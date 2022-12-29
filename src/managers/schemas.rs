use crate::managers::errors::Error;
use futures::{Stream, TryStreamExt};
use log::{debug, error, info};
use sqlx::types::Uuid;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::Status;

use crate::api;

use api::catalog::Schema;

#[derive(sqlx::FromRow)]
struct SchemaRow {
    id: Uuid,
    kind: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    data: serde_json::Value,
}

#[derive(Debug)]
pub struct Manager {
    pool: Arc<sqlx::PgPool>,
}

impl Manager {
    pub async fn new(pool: Arc<sqlx::PgPool>) -> Result<Manager, Error> {
        let res = Manager { pool };
        res.init_tables().await?;
        Ok(res)
    }

    pub async fn load_from_directory(&self, directory: &str) -> Result<(), Error> {
        let mut files = std::fs::read_dir(directory)?;
        for entry in files.by_ref() {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_str().unwrap();
                if filename.ends_with(".json") {
                    let kind = filename.trim_end_matches(".json").to_string();
                    let data = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
                    match self.create(&kind, &data).await {
                        Ok(_) => info!("created schema {}", kind),
                        Err(_) => {
                            match self.get_by_kind(&kind).await {
                                Ok(old) => {
                                    self.update(&Uuid::parse_str(&old.id)?, &data).await?;
                                    info!("updated schema {}", kind);
                                },
                                Err(e) => error!("failed to retrieve old schema {}: {}", kind, e),
                            };
                        },
                    };
                }
            }
        }

        Ok(())
    }

    #[tracing::instrument(name = "mgr::schemas::init_tables", skip(self))]
    async fn init_tables(&self) -> Result<(), Error> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS
            schemas(
                id UUID PRIMARY KEY,
                kind TEXT UNIQUE,
                created_at TIMESTAMPTZ DEFAULT now(),
                updated_at TIMESTAMPTZ DEFAULT now(),
                data JSONB
            )"#,
        )
        .execute(self.pool.deref())
        .await?;
        Ok(())
    }

    #[tracing::instrument(name = "mgr::schemas::create", skip(self))]
    pub async fn create(&self, kind: &str, data: &serde_json::Value) -> Result<Schema, Error> {
        let id = Uuid::from_bytes(uuid::Uuid::new_v4().into_bytes());

        let now = chrono::Utc::now();

        let data_bytes = match serde_json::to_string(data) {
            Ok(data) => data,
            Err(err) => {
                return Err(Error::InvalidArgument(format!(
                    "failed to parse schema data: {}",
                    err
                )));
            }
        };

        let res = Schema {
            id: id.to_hyphenated().to_string(),
            kind: kind.to_string(),
            data: data_bytes,
            created_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
        };

        match sqlx::query("INSERT INTO schemas(id, kind, created_at, updated_at, data) VALUES($1, $2, $3, $4, $5)").
            bind(&id).
            bind(&res.kind).
            bind(now).
            bind(now).
            bind(&data).
            execute(self.pool.deref()).await {
                Ok(_) => (),
                Err(err) => {
                    return Err(Error::Database(format!("failed to create Schema: {}", err)));
                }
            };

        self.create_unique_indexes(kind, data).await?;

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::schemas::get", skip(self))]
    pub async fn get(&self, id: &Uuid) -> Result<Schema, Error> {
        let row: SchemaRow = match sqlx::query_as(
            "SELECT id, kind, created_at, updated_at, data FROM schemas WHERE id = $1",
        )
        .bind(id)
        .fetch_one(self.pool.deref())
        .await
        {
            Ok(row) => row,
            Err(_) => {
                return Err(Error::NotFound);
            }
        };

        let data = match serde_json::to_string(&row.data) {
            Ok(data) => data,
            Err(err) => {
                return Err(Error::Database(format!(
                    "failed to encode schema data: {}",
                    err
                )));
            }
        };

        let res = Schema {
            id: row.id.to_hyphenated().to_string(),
            kind: row.kind,
            data,
            created_at: Some(prost_types::Timestamp {
                seconds: row.created_at.timestamp(),
                nanos: 0,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: row.updated_at.timestamp(),
                nanos: 0,
            }),
        };

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::schemas::get_by_kind", skip(self))]
    pub async fn get_by_kind(&self, kind: &str) -> Result<Schema, Error> {
        let row: SchemaRow = match sqlx::query_as(
            "SELECT id, kind, created_at, updated_at, data FROM schemas WHERE kind = $1",
        )
        .bind(kind)
        .fetch_one(self.pool.deref())
        .await
        {
            Ok(row) => row,
            Err(_) => {
                return Err(Error::NotFound);
            }
        };

        let data = match serde_json::to_string(&row.data) {
            Ok(data) => data,
            Err(err) => {
                return Err(Error::Database(format!(
                    "failed to encode schema data: {}",
                    err
                )));
            }
        };

        let res = Schema {
            id: row.id.to_hyphenated().to_string(),
            kind: row.kind,
            data,
            created_at: Some(prost_types::Timestamp {
                seconds: row.created_at.timestamp(),
                nanos: 0,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: row.updated_at.timestamp(),
                nanos: 0,
            }),
        };

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::schemas::delete", skip(self))]
    pub async fn delete(&self, id: &Uuid) -> Result<Schema, Error> {
        let res = self.get(id).await?;

        match sqlx::query("DELETE FROM schemas WHERE id = $1")
            .bind(&id)
            .execute(self.pool.deref())
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(Error::Database(format!("failed to delete Schema: {}", err)));
            }
        };

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::schemas::update", skip(self))]
    pub async fn update(&self, id: &Uuid, doc: &serde_json::Value) -> Result<Schema, Error> {
        let mut schema = self.get(id).await?;

        let mut data = match serde_json::from_str(&schema.data) {
            Ok(obj) => obj,
            Err(err) => {
                return Err(Error::InvalidArgument(format!(
                    "failed to parse schema object: {}",
                    err
                )));
            }
        };

        let now = chrono::Utc::now();

        json_patch::merge(&mut data, doc);

        match sqlx::query("UPDATE schemas SET data = $1, updated_at = $2 WHERE id = $3")
            .bind(&data)
            .bind(&now)
            .bind(id)
            .execute(self.pool.deref())
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(Error::Database(format!("failed to update schema: {}", err)));
            }
        };

        schema.data = match serde_json::to_string(&data) {
            Ok(data) => data,
            Err(err) => {
                return Err(Error::Database(format!(
                    "failed encode data object: {}",
                    err
                )));
            }
        };

        schema.updated_at = Some(prost_types::Timestamp {
            seconds: now.timestamp(),
            nanos: 0,
        });

        Ok(schema)
    }

    #[tracing::instrument(name = "mgr::schemas::list", skip(self))]
    pub async fn list(
        &self,
    ) -> Result<Pin<Box<impl Stream<Item = Result<Schema, Status>> + Send + Sync + 'static>>, Status>
    {
        let (tx, rx) = mpsc::channel(4);
        let pool = self.pool.clone();
        tokio::spawn(async move {
            let mut rows =
                sqlx::query_as("SELECT id, kind, created_at, updated_at, data FROM schemas;")
                    .fetch(pool.deref());
            loop {
                let row: SchemaRow = match rows.try_next().await {
                    Ok(row) => match row {
                        Some(row) => row,
                        None => {
                            break;
                        }
                    },
                    Err(err) => {
                        return Err(Error::Database(format!("failed to get schema: {}", err)));
                    }
                };
                let data = match serde_json::to_string(&row.data) {
                    Ok(data) => data,
                    Err(err) => {
                        return Err(Error::Database(format!(
                            "failed to encode schema data: {}",
                            err
                        )));
                    }
                };
                let res = Schema {
                    id: row.id.to_hyphenated().to_string(),
                    kind: row.kind,
                    data,
                    created_at: Some(prost_types::Timestamp {
                        seconds: row.created_at.timestamp(),
                        nanos: 0,
                    }),
                    updated_at: Some(prost_types::Timestamp {
                        seconds: row.updated_at.timestamp(),
                        nanos: 0,
                    }),
                };
                match tx.send(Ok(res)).await {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(Error::Database(format!("failed to send Schema: {}", err)));
                    }
                };
            }
            Ok(())
        });

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    #[tracing::instrument(name = "mgr::schemas::validate", skip(self))]
    pub async fn validate(&self, kind: &str, doc: &serde_json::Value) -> Result<(), Error> {
        let row: SchemaRow = match sqlx::query_as(
            "SELECT id, kind, created_at, updated_at, data FROM schemas WHERE kind = $1",
        )
        .bind(kind)
        .fetch_one(self.pool.deref())
        .await
        {
            Ok(row) => row,
            Err(_) => {
                return Err(Error::NotFound);
            }
        };

        let schema = jsonschema::JSONSchema::options()
            .with_draft(jsonschema::Draft::Draft7)
            .compile(&row.data)?;

        let result = schema.validate(doc);
        if let Err(errors) = result {
            let mut msg = "failed to validate data against schema:".to_owned();
            errors.for_each(|err| {
                msg.push_str("\n* ");
                msg.push_str(&err.to_string());
            });
            return Err(Error::InvalidArgument(msg));
        }

        Ok(())
    }

    #[tracing::instrument(name = "mgr::schemas::create_unique_indexes", skip(self))]
    async fn create_unique_indexes(
        &self,
        kind: &str,
        doc: &serde_json::Value,
    ) -> Result<(), Error> {
        info!("create unique indexes");
        match doc {
            serde_json::Value::Object(obj) => {
                debug!("schema is a object");
                if let serde_json::Value::Object(obj) = &obj["properties"] {
                    debug!("properties is a object");
                    for (prop_name, value) in obj {
                        debug!("inspect prop {}", prop_name);
                        if let serde_json::Value::Object(prop_spec) = value {
                            debug!("prop spec is a object");
                            for (key, value) in prop_spec {
                                if key == "x-unique" && *value == serde_json::Value::Bool(true) {
                                    debug!("found x-unique entry, create a new index...");
                                    let index_name = format!("{}_{}", kind, prop_name);
                                    let q = format!("CREATE UNIQUE INDEX {} ON resources ((data->>'{}')) WHERE kind = '{}';", index_name, prop_name, kind);
                                    match sqlx::query(&q).execute(self.pool.deref()).await {
                                        Ok(_) => {
                                            info!("created unique index {}", &index_name);
                                        }
                                        Err(err) => {
                                            error!("failed to create unqiue index {}", err);
                                            return Err(Error::Database(format!(
                                                "failed to create unique index: {}",
                                                err
                                            )));
                                        }
                                    };
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                return Err(Error::InvalidArgument(
                    "failed to create unique index: data must be an object".to_owned(),
                ));
            }
        };

        Ok(())
    }
}
