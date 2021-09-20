use futures::{Stream, TryStreamExt};
use pwhash::bcrypt;
use sqlx::types::Uuid;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::Status;

use crate::api;
use crate::token;

use api::idp::{CreateServiceAccountResponse, ServiceAccount, UpdateServiceAccountResponse};
use token::Claims;

#[derive(sqlx::FromRow)]
struct ServiceAccountRow {
    id: Uuid,
    name: String,
    is_admin: bool,
    secret_key_hash: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct Manager {
    pool: Arc<sqlx::PgPool>,
}

impl Manager {
    pub async fn new(pool: Arc<sqlx::PgPool>) -> Result<Manager, sqlx::Error> {
        let res = Manager { pool };
        res.init_tables().await?;
        Ok(res)
    }

    #[tracing::instrument(skip(self))]
    async fn init_tables(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS
            service_accounts(
                id UUID PRIMARY KEY,
                name TEXT,
                is_admin BOOL,
                secret_key_hash TEXT,
                created_at TIMESTAMPTZ DEFAULT now(),
                updated_at TIMESTAMPTZ DEFAULT now()
            )",
        )
        .execute(self.pool.deref())
        .await?;
        Ok(())
    }

    #[tracing::instrument(name = "mgr::service_accounts::create", skip(self))]
    pub async fn create(
        &self,
        claims: &Claims,
        name: &str,
        is_admin: bool,
    ) -> Result<CreateServiceAccountResponse, Status> {
        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins can create new service_accounts",
            ));
        }

        let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, name.as_bytes());

        let now = chrono::Utc::now();

        let password = self.generate_password().await;

        let sa = ServiceAccount {
            id: id.to_hyphenated().to_string(),
            name: name.to_string(),
            is_admin,
            secret_key_hash: bcrypt::hash(&password).unwrap(),
            created_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
        };

        match sqlx::query("INSERT INTO service_accounts(id, name, is_admin, secret_key_hash, created_at, updated_at) VALUES($1, $2, $3, $4, $5, $6)").
            bind(&id).
            bind(&sa.name).
            bind(&sa.is_admin).
            bind(&sa.secret_key_hash).
            bind(now).
            bind(now).
            execute(self.pool.deref()).await {
                Ok(_) => (),
                Err(err) => {
                    return Err(Status::internal(format!("failed to create ServiceAccount: {}", err)));
                }
            };

        Ok(CreateServiceAccountResponse {
            id: sa.id,
            name: sa.name,
            secret_key: password,
            is_admin: sa.is_admin,
            created_at: sa.created_at,
            updated_at: sa.updated_at,
        })
    }

    #[tracing::instrument(name = "mgr::service_accounts::get", skip(self))]
    pub async fn get(&self, claims: &Claims, id: &Uuid) -> Result<ServiceAccount, Status> {
        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins can read service_accounts",
            ));
        }

        let row: ServiceAccountRow = match sqlx::query_as("SELECT id, name, is_admin, secret_key_hash, created_at, updated_at FROM service_accounts WHERE id = $1").
                bind(id).
                fetch_one(self.pool.deref()).await {
            Ok(row) => row,
            Err(err) => {
                return Err(Status::not_found(format!("failed to find ServiceAccount: {}", err)));
            },
        };

        let res = ServiceAccount {
            id: row.id.to_hyphenated().to_string(),
            name: row.name,
            is_admin: row.is_admin,
            secret_key_hash: row.secret_key_hash,
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

    #[tracing::instrument(name = "mgr::service_accounts::delete", skip(self))]
    pub async fn delete(&self, claims: &Claims, id: &Uuid) -> Result<ServiceAccount, Status> {
        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins can delete service_accounts",
            ));
        }

        let res = self.get(claims, id).await?;

        match sqlx::query("DELETE FROM ServiceAccounts WHERE id = $1")
            .bind(id)
            .execute(self.pool.deref())
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(Status::internal(format!(
                    "failed to delete ServiceAccount: {}",
                    err
                )));
            }
        };

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::service_accounts::update", skip(self))]
    pub async fn update(
        &self,
        claims: &Claims,
        id: &Uuid,
    ) -> Result<UpdateServiceAccountResponse, Status> {
        if !claims.adm && claims.sub != id.to_hyphenated().to_string() {
            return Err(Status::permission_denied(
                "only admins or the service_account itself can update a service_account",
            ));
        }

        let sa = self.get(claims, id).await?;

        let now = chrono::Utc::now();

        let password = self.generate_password().await;

        let password_hash = bcrypt::hash(&password).unwrap();

        match sqlx::query(
            "UPDATE service_accounts SET secret_key_hash = $1, updated_at = $2 WHERE id = $3",
        )
        .bind(&password_hash)
        .bind(&now)
        .bind(id)
        .execute(self.pool.deref())
        .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(Status::internal(format!(
                    "failed to update ServiceAccount: {}",
                    err
                )));
            }
        }

        Ok(UpdateServiceAccountResponse {
            id: sa.id,
            name: sa.name,
            is_admin: sa.is_admin,
            secret_key: password,
            created_at: sa.created_at,
            updated_at: sa.updated_at,
        })
    }

    #[tracing::instrument(name = "mgr::service_accounts::list", skip(self))]
    pub async fn list(
        &self,
    ) -> Result<
        Pin<Box<impl Stream<Item = Result<ServiceAccount, Status>> + Send + Sync + 'static>>,
        Status,
    > {
        let (tx, rx) = mpsc::channel(4);
        let pool = self.pool.clone();
        tokio::spawn(async move {
            let mut rows = sqlx::query_as("SELECT id, is_admin, secret_key_hash, created_at, updated_at FROM service_accounts;").fetch(pool.deref());
            loop {
                let row: ServiceAccountRow = match rows.try_next().await {
                    Ok(row) => match row {
                        Some(row) => row,
                        None => {
                            break;
                        }
                    },
                    Err(err) => {
                        return Err(Status::internal(format!(
                            "failed to update ServiceAccount: {}",
                            err
                        )));
                    }
                };
                let res = ServiceAccount {
                    id: row.id.to_hyphenated().to_string(),
                    name: row.name,
                    is_admin: row.is_admin,
                    secret_key_hash: row.secret_key_hash,
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
                        return Err(Status::internal(format!(
                            "failed to send ServiceAccount: {}",
                            err
                        )));
                    }
                };
            }
            Ok(())
        });

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    #[tracing::instrument(name = "mgr::service_accounts::generate_password", skip(self))]
    async fn generate_password(&self) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(*&^%$#@!~";
        const PASSWORD_LEN: usize = 32;
        let mut rng = rand::thread_rng();

        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        password
    }
}
