use super::errors::Error;
use crate::api;

use futures::Stream;
use sqlx::{Postgres, Transaction};
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
        res.init_table().await?;
        Ok(res)
    }

    async fn init_table(&self) -> Result<(), Error> {
        let _ = sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS locks (
                id VARCHAR(255) PRIMARY KEY,
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
                fencing_token BIGINT NOT NULL
            );
        "#,
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
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

        let fencing_token = self.get_fencing_token(&lock_id).await?;

        tokio::spawn(
            async move {
                use tokio::time::{sleep, Duration};
                loop {
                    match tx
                        .send(Ok(LockResponse {
                            lock_id: lock_id.to_string(),
                            fencing_token,
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

        let fencing_token = self.get_fencing_token(&lock_id).await?;

        tokio::spawn(
            async move {
                use tokio::time::{sleep, Duration};
                loop {
                    match tx
                        .send(Ok(LockResponse {
                            lock_id: lock_id.to_string(),
                            fencing_token,
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

    #[tracing::instrument(name = "mgr::locks::check_fencing_token", skip(self))]
    pub async fn check_fencing_token(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        lock_id: &str,
        fencing_token: i64,
    ) -> Result<bool, Error> {
        let row: (bool,) =
            match sqlx::query_as("SELECT (fencing_token = $1) FROM locks WHERE id = $2")
                .bind(fencing_token)
                .bind(lock_id)
                .fetch_one(tx)
                .await
            {
                Ok(row) => row,
                Err(err) => return Err(Error::Database(err.to_string())),
            };

        Ok(row.0)
    }

    #[tracing::instrument(name = "mgr::locks::get_fencing_token", skip(self))]
    async fn get_fencing_token(&self, lock_id: &str) -> Result<i64, Status> {
        // insert lock into db or increase fencing_token if lock already exists
        // This is done outside of the transaction to make the new fencing token available to other
        // Note that this is done AFTER the pg_advisory_lock is aquired.
        let row: (i64,) = match sqlx::query_as(
            r#"
            INSERT INTO locks (id, fencing_token) 
            VALUES ($1, $2) 
            ON CONFLICT (id) DO UPDATE 
            SET fencing_token = locks.fencing_token + 1,
                updated_at = NOW()
            RETURNING fencing_token"#,
        )
        .bind(&lock_id)
        .bind(1)
        .fetch_one(&*self.pool)
        .await
        {
            Ok(row) => row,
            Err(err) => return Err(Status::internal(err.to_string())),
        };
        Ok(row.0)
    }
}
