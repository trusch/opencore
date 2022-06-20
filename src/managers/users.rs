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

use api::idp::User;
use token::Claims;

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    name: String,
    external_id: String,
    is_admin: bool,
    password_hash: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub enum GetSelector {
    ByExternalId(String),
    ByEmail(String),
    ByDid(String),
    ById(Uuid),
}

#[derive(Clone, Debug)]
pub struct Manager {
    pool: Arc<sqlx::PgPool>,
}

impl Manager {
    pub async fn new(pool: Arc<sqlx::PgPool>) -> Result<Manager, sqlx::Error> {
        let res = Manager { pool };
        res.init_tables().await?;
        Ok(res)
    }

    async fn init_tables(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS
            users(
                id UUID PRIMARY KEY,
                name TEXT UNIQUE,
                external_id TEXT UNIQUE,
                is_admin BOOL,
                password_hash TEXT,
                created_at TIMESTAMPTZ DEFAULT now(),
                updated_at TIMESTAMPTZ DEFAULT now()
            )",
        )
        .execute(self.pool.deref())
        .await?;
        Ok(())
    }

    #[tracing::instrument(name = "mgr::users::create", skip(self))]
    pub async fn create(
        &self,
        claims: &Claims,
        name: &str,
        external_id: &str,
        is_admin: &bool,
        password: &str,
    ) -> Result<User, Status> {
        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins can create new users",
            ));
        }

        let id = Uuid::from_bytes(uuid::Uuid::new_v4().into_bytes());

        let now = chrono::Utc::now();

        let res = User {
            id: id.to_hyphenated().to_string(),
            name: name.to_string(),
            external_id: external_id.to_string(),
            is_admin: *is_admin,
            password_hash: bcrypt::hash(password).unwrap(),
            created_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
        };

        match sqlx::query("INSERT INTO users(id, name, external_id, is_admin, password_hash, created_at, updated_at) VALUES($1, $2, $3, $4, $5, $6, $7)").
            bind(&id).
            bind(&res.name).
            bind(&res.external_id).
            bind(&res.is_admin).
            bind(&res.password_hash).
            bind(now).
            bind(now).
            execute(self.pool.deref()).await {
                Ok(_) => (),
                Err(err) => {
                    return Err(Status::internal(format!("failed to create User: {}", err)));
                }
            };

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::users::get", skip(self))]
    pub async fn get(&self, claims: &Claims, selector: GetSelector) -> Result<User, Status> {
        
        let filter: String;
        let argument: String;

        match selector {
            GetSelector::ByExternalId(id) | GetSelector::ByEmail(id)|  GetSelector::ByDid(id) => {
                filter = "external_id = $1".into();
                argument = id;
            }
            GetSelector::ById(id) => {
                filter = "id::TEXT = $1".into();
                argument = id.to_hyphenated().to_string();
            }
        };

        let query = format!("SELECT id, name, external_id, is_admin, password_hash, created_at, updated_at FROM users WHERE {}", filter);

        let row: UserRow = match sqlx::query_as(&query)
            .bind(&argument)
            .fetch_one(self.pool.deref())
            .await
        {
            Ok(row) => row,
            Err(err) => {
                return Err(Status::not_found(format!("failed to find User: {}", err)));
            }
        };

        let res = User {
            id: row.id.to_hyphenated().to_string(),
            name: row.name,
            external_id: row.external_id,
            is_admin: row.is_admin,
            password_hash: row.password_hash,
            created_at: Some(prost_types::Timestamp {
                seconds: row.created_at.timestamp(),
                nanos: 0,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: row.updated_at.timestamp(),
                nanos: 0,
            }),
        };

        if !claims.adm && !(claims.sub == res.id) {
            return Err(Status::permission_denied("you are not allowed to get this user"))
        }

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::users::delete", skip(self))]
    pub async fn delete(&self, claims: &Claims, id: &Uuid) -> Result<User, Status> {
        if !claims.adm {
            return Err(Status::permission_denied("only admins can delete users"));
        }

        let res = self.get(claims, GetSelector::ById(*id)).await?;

        match sqlx::query("DELETE FROM Users WHERE id = $1")
            .bind(&id)
            .execute(self.pool.deref())
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(Status::internal(format!("failed to delete User: {}", err)));
            }
        };

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::users::update", skip(self))]
    pub async fn update(
        &self,
        claims: &Claims,
        id: &Uuid,
        name: &str,
        password: &str,
    ) -> Result<User, Status> {
        if !claims.adm && claims.sub != id.to_hyphenated().to_string() {
            return Err(Status::permission_denied(
                "only admins or the user itself can update a users",
            ));
        }

        let mut user = self.get(claims, GetSelector::ById(*id)).await?;

        let now = chrono::Utc::now();

        let name = if !name.is_empty() { name } else { &user.name };
        let password_hash = if !password.is_empty() {
            bcrypt::hash(password).unwrap()
        } else {
            user.password_hash.clone()
        };

        match sqlx::query("UPDATE users SET name = $1, password_hash: $2 updated_at = $3 WHERE id = $4").
            bind(&name).
            bind(&password_hash).
            bind(&now).
            bind(id).
            execute(self.pool.deref()).await {
            Ok(_) => (),
            Err(err) => {
                return Err(Status::internal(format!("failed to update User: {}", err)));
            },
        }

        user.name = name.to_string();
        user.password_hash = password_hash;
        user.updated_at = Some(prost_types::Timestamp {
            seconds: now.timestamp(),
            nanos: 0,
        });

        Ok(user)
    }

    #[tracing::instrument(name = "mgr::users::list", skip(self))]
    pub async fn list(
        &self,
    ) -> Result<Pin<Box<impl Stream<Item = Result<User, Status>> + Send + Sync + 'static>>, Status>
    {
        let (tx, rx) = mpsc::channel(4);
        let pool = self.pool.clone();
        tokio::spawn(async move {
            let mut rows = sqlx::query_as("SELECT id, name, external_id, is_admin, password_hash, created_at, updated_at FROM users;").fetch(pool.deref());
            loop {
                let row: UserRow = match rows.try_next().await {
                    Ok(row) => match row {
                        Some(row) => row,
                        None => {
                            break;
                        }
                    },
                    Err(err) => {
                        return Err(Status::internal(format!("failed to update User: {}", err)));
                    }
                };
                let res = User {
                    id: row.id.to_hyphenated().to_string(),
                    name: row.name,
                    external_id: row.external_id,
                    is_admin: row.is_admin,
                    password_hash: row.password_hash,
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
                        return Err(Status::internal(format!("failed to send User: {}", err)));
                    }
                };
            }
            Ok(())
        });

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
