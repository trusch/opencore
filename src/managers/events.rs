use super::errors::Error;

use futures::Stream;
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tracing::Instrument;

use crate::api::catalog;
use crate::managers;
use crate::token::Claims;

use catalog::Event;

#[derive(sqlx::FromRow, Debug)]
struct EventRow {
    id: Uuid,
    serial: i64,
    resource_id: Uuid,
    resource_kind: String,
    event_type: i32,
    data: serde_json::Value,
    labels: serde_json::Value,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct SerializableEventRow {
    id: String,
    serial: i64,
    resource_id: String,
    resource_kind: String,
    event_type: i32,
    data: serde_json::Value,
    labels: serde_json::Value,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<EventRow> for SerializableEventRow {
    fn from(row: EventRow) -> Self {
        SerializableEventRow {
            id: row.id.to_hyphenated().to_string(),
            serial: row.serial,
            resource_id: row.resource_id.to_hyphenated().to_string(),
            resource_kind: row.resource_kind,
            labels: row.labels,
            event_type: row.event_type as i32,
            data: row.data,
            created_at: row.created_at,
        }
    }
}

pub struct Manager {
    pool: Arc<sqlx::PgPool>,
    permissions: Arc<managers::permissions::Manager>,
    sender: tokio::sync::broadcast::Sender<Event>,
}

#[derive(Debug, Clone)]
pub enum SubscribeFilter {
    ByType(catalog::EventType),
    ByKind(String),
    ByResource(String),
}

impl std::fmt::Debug for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventManager")
            .field("pool", &self.pool)
            .finish()
    }
}

impl Manager {
    pub async fn new(
        pool: Arc<sqlx::PgPool>,
        permissions: Arc<managers::permissions::Manager>,
        db_connect_str: &str,
    ) -> Result<Manager, Error> {
        let (tx, _) = tokio::sync::broadcast::channel(16);
        let res = Manager {
            pool,
            permissions,
            sender: tx,
        };
        res.init_tables().await?;
        let mut listener = sqlx::postgres::PgListener::connect(&db_connect_str).await?;
        listener.listen_all(vec!["event"]).await?;
        let sender = res.sender.to_owned();
        tokio::spawn(async move {
            loop {
                let notification = match listener.recv().await {
                    Ok(n) => n,
                    Err(err) => {
                        log::error!("failed to receive events from db: {}", err);
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        continue;
                    }
                };
                let row: SerializableEventRow = match serde_json::from_str(notification.payload()) {
                    Ok(row) => row,
                    Err(err) => {
                        log::error!("failed to parse events from db: {}", err);
                        continue;
                    }
                };

                let data_str = match serde_json::to_string(&row.data) {
                    Ok(d) => d,
                    Err(err) => {
                        log::error!("failed to parse data from event: {}", err);
                        continue;
                    }
                };

                log::info!(
                    "publish {:?} event for resource {}",
                    row.event_type,
                    row.resource_id
                );

                match sender.send(Event {
                    id: row.id,
                    resource_id: row.resource_id,
                    resource_kind: row.resource_kind,
                    resource_labels: serde_json::from_value(row.labels).unwrap(),
                    event_type: row.event_type,
                    data: data_str,
                    created_at: Some(prost_types::Timestamp {
                        seconds: row.created_at.timestamp(),
                        nanos: 0,
                    }),
                }) {
                    Ok(_) => (),
                    Err(err) => {
                        log::error!("failed to send event: {}", err);
                        continue;
                    }
                }
            }
        });
        Ok(res)
    }

    #[tracing::instrument(skip(self))]
    async fn init_tables(&self) -> Result<(), Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS
            events(
                id UUID,
                serial BIGSERIAL,
                resource_id UUID,
                resource_kind TEXT,
                event_type INT4,
                data JSONB,
                labels JSONB,
                created_at TIMESTAMPTZ DEFAULT now()
            )",
        )
        .execute(self.pool.deref())
        .await?;
        Ok(())
    }

    #[tracing::instrument(name = "mgr::events::publish", skip(self))]
    pub async fn publish(
        &self,
        claims: &Claims,
        resource_id: &Uuid,
        resource_kind: &String,
        resource_labels: &HashMap<String, String>,
        event_type: catalog::EventType,
        data: &serde_json::Value,
    ) -> Result<Event, Error> {
        if !claims.adm {
            return Err(Error::Forbidden);
        }

        let event_id = Uuid::new_v4();

        let now = chrono::Utc::now();

        let mut tx = self.pool.begin().await?;

        let row: EventRow = sqlx::query_as(
            r#"INSERT INTO events(
            id,
            resource_id,
            resource_kind,
            event_type,
            data,
            labels,
            created_at) VALUES($1, $2, $3, $4, $5, $6, $7)
            RETURNING events.*"#,
        )
        .bind(&event_id)
        .bind(resource_id)
        .bind(resource_kind)
        .bind(event_type as i32)
        .bind(&data)
        .bind(serde_json::to_value(resource_labels)?)
        .bind(now)
        .fetch_one(&mut tx)
        .await?;

        let mut serializable_row = SerializableEventRow::from(row);
        let mut payload = serde_json::to_string(&serializable_row)?;
        if payload.len() > 8000 {
            // This will be rejected by postgres otherwise, so we trucate it by setting the data part to null.
            // If a client sees this null, it indicates that they should fetch the resource to get the data.
            serializable_row.data = serde_json::Value::Null;
            payload = serde_json::to_string(&serializable_row)?;
        }
        sqlx::query(r#"SELECT pg_notify($1, $2)"#)
            .bind("event")
            .bind(payload)
            .execute(&mut tx)
            .await?;
        tx.commit().await?;

        Ok(Event {
            id: event_id.to_hyphenated().to_string(),
            resource_id: resource_id.to_hyphenated().to_string(),
            resource_kind: resource_kind.clone(),
            resource_labels: resource_labels.clone(),
            event_type: event_type as i32,
            data: serde_json::to_string(data)?,
            created_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
        })
    }

    #[tracing::instrument(name = "mgr::events::subscribe", skip(self))]
    pub async fn subscribe(
        &self,
        claims: &Claims,
        filters: &Vec<SubscribeFilter>,
    ) -> Result<
        Pin<Box<impl Stream<Item = Result<Event, tonic::Status>> + Send + Sync + 'static>>,
        tonic::Status,
    > {
        use tokio_stream::wrappers::BroadcastStream;
        let mut stream = BroadcastStream::new(self.sender.subscribe());
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        let perms = self.permissions.clone();
        let claims = claims.clone();
        let filters = filters.to_vec();
        tracing::info!(
            "subscribe to events with claims: {:?} filters: {:?}",
            &claims,
            &filters
        );
        tokio::spawn(
            async move {
                use tokio_stream::StreamExt;
                'mainloop: while let Some(evt) = stream.next().await {
                    let event = match evt {
                        Ok(event) => event,
                        Err(err) => {
                            log::error!("failed to receive events from db: {}", err);
                            continue;
                        }
                    };
                    tracing::info!(
                        "got event ({:?}) in subscriber {:?}, doing checks",
                        &event,
                        &claims.sub
                    );
                    if !claims.adm {
                        let id = match Uuid::parse_str(&event.resource_id) {
                            Ok(id) => id,
                            Err(_) => continue,
                        };
                        match perms.check(&id, "read", &claims).await {
                            Ok(_) => {}
                            Err(_) => {
                                tracing::info!(
                                    "discard event {:?} for {:?} because of insufficent privileges",
                                    &event,
                                    &claims.sub
                                );
                                continue;
                            }
                        };
                    }

                    for filter in filters.iter() {
                        match filter {
                            SubscribeFilter::ByResource(id) => {
                                if event.resource_id != *id {
                                    continue 'mainloop;
                                }
                            }
                            SubscribeFilter::ByKind(kind) => {
                                if event.resource_kind != *kind {
                                    continue 'mainloop;
                                }
                            }
                            SubscribeFilter::ByType(event_type) => {
                                let t = catalog::EventType::from(event.event_type);
                                if t != *event_type {
                                    continue 'mainloop;
                                }
                            }
                        }
                    }

                    tracing::info!("all checks passed, send out event");

                    match tx.send(Ok(event)).await {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("error while sending: {}", err);
                            break;
                        }
                    };
                }
                tracing::info!("end event stream loop");
            }
            .instrument(tracing::debug_span!("stream_events")),
        );
        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
