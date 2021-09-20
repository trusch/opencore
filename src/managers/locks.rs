use super::errors::Error;
use crate::api;

use futures::Stream;
use std::pin::Pin;
use tracing::Instrument;

use log::info;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::Status;

use api::catalog::LockResponse;

#[derive(Debug)]
pub struct Manager {
    pool: Arc<sqlx::PgPool>,
}
impl Manager {
    pub async fn new(pool: Arc<sqlx::PgPool>) -> Result<Manager, Error> {
        let res = Manager { pool };
        Ok(res)
    }

    #[tracing::instrument(name = "mgr::locks::lock", skip(self))]
    pub async fn lock(
        &self,
        lock_id: &str,
    ) -> Result<
        Pin<Box<impl Stream<Item = Result<LockResponse, Status>> + Send + Sync + 'static>>,
        Status,
    > {
        info!("call lock");
        let (tx, rx) = mpsc::channel(4);
        let lock_id = lock_id.to_string();

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut s = DefaultHasher::new();
        lock_id.hash(&mut s);
        let lock_id_int = s.finish();

        let mut transaction = match self.pool.begin().await {
            Ok(t) => t,
            Err(err) => return Err(Status::internal(err.to_string())),
        };
        match sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(lock_id_int as i64)
            .execute(&mut transaction)
            .await
        {
            Ok(row) => row,
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        info!("got lock");

        tokio::spawn(
            async move {
                use tokio::time::{sleep, Duration};
                loop {
                    match tx
                        .send(Ok(LockResponse {
                            lock_id: lock_id.to_string(),
                        }))
                        .await
                    {
                        Ok(_) => {
                            sleep(Duration::from_millis(100)).await;
                        }
                        Err(err) => {
                            log::error!("failed to send permission info: {}", err);
                            break;
                        }
                    };
                }

                match transaction.commit().await {
                    Ok(_) => info!("released lock"),
                    Err(_) => info!("failed to commit lock transaction"),
                };
            }
            .instrument(tracing::debug_span!("stream_lock_messages")),
        );

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    #[tracing::instrument(name = "mgr::locks::try_lock", skip(self))]
    pub async fn try_lock(
        &self,
        lock_id: &str,
    ) -> Result<
        Pin<Box<impl Stream<Item = Result<LockResponse, Status>> + Send + Sync + 'static>>,
        Status,
    > {
        info!("call try_lock");

        let (tx, rx) = mpsc::channel(4);
        let lock_id = lock_id.to_string();

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();
        lock_id.hash(&mut s);
        let lock_id_int = s.finish();

        let mut transaction = match self.pool.begin().await {
            Ok(t) => t,
            Err(err) => return Err(Status::internal(err.to_string())),
        };
        let row: (bool,) = match sqlx::query_as("SELECT pg_try_advisory_xact_lock($1)")
            .bind(lock_id_int as i64)
            .fetch_one(&mut transaction)
            .await
        {
            Ok(row) => row,
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        if !row.0 {
            return Err(Status::resource_exhausted("failed to get lock"));
        }

        info!("got lock");

        tokio::spawn(
            async move {
                use tokio::time::{sleep, Duration};
                loop {
                    match tx
                        .send(Ok(LockResponse {
                            lock_id: lock_id.to_string(),
                        }))
                        .await
                    {
                        Ok(_) => {
                            sleep(Duration::from_millis(1000)).await;
                        }
                        Err(err) => {
                            log::error!("failed to send permission info: {}", err);
                            break;
                        }
                    };
                }
                match transaction.commit().await {
                    Ok(_) => info!("released lock"),
                    Err(_) => info!("failed to commit lock transaction"),
                };
            }
            .instrument(tracing::debug_span!("stream_lock_messages")),
        );

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
