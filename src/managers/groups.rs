use futures::{Stream, TryStreamExt};
use sqlx::types::Uuid;
use sqlx::Row;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::Status;

use crate::api;
use crate::managers;
use crate::token;

use api::idp::{Group, GroupMember};

#[derive(sqlx::FromRow)]
struct GroupRow {
    id: Uuid,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
struct GroupMemberRow {
    id: Uuid,
    name: String,
    email: String,
    is_admin: bool,
    joined_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct Manager {
    pool: Arc<sqlx::PgPool>,
    users_mgr: Arc<managers::users::Manager>,
}

impl Manager {
    pub async fn new(
        pool: Arc<sqlx::PgPool>,
        users_mgr: Arc<managers::users::Manager>,
    ) -> Result<Manager, sqlx::Error> {
        let res = Manager {
            pool: pool,
            users_mgr: users_mgr,
        };
        res.init_tables().await?;
        Ok(res)
    }

    #[tracing::instrument(name = "mgr::groups::init_tables", skip(self))]
    async fn init_tables(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS
            groups(
                id UUID PRIMARY KEY,
                name TEXT,
                created_at TIMESTAMPTZ DEFAULT now(),
                updated_at TIMESTAMPTZ DEFAULT now()
            )",
        )
        .execute(self.pool.deref())
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS
            group_members(
                group_id UUID REFERENCES groups(id) ON DELETE CASCADE,
                user_id UUID REFERENCES users(id) ON DELETE CASCADE,
                is_admin BOOL DEFAULT false,
                created_at TIMESTAMPTZ DEFAULT now(),
                PRIMARY KEY(group_id, user_id)
            )",
        )
        .execute(self.pool.deref())
        .await?;
        Ok(())
    }

    #[tracing::instrument(name = "mgr::groups::create", skip(self))]
    pub async fn create(&self, claims: &token::Claims, name: &String) -> Result<Group, Status> {
        let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, &name.as_bytes());
        let now = chrono::Utc::now();

        let requester_id = match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(err) => {
                return Err(Status::permission_denied(format!(
                    "failed to parse user id from token: {}",
                    err
                )))
            }
        };

        let res = Group {
            id: id.to_hyphenated().to_string(),
            name: name.clone(),
            created_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
        };

        match sqlx::query(
            "INSERT INTO groups(id, name, created_at, updated_at) VALUES($1, $2, $3, $4)",
        )
        .bind(&id)
        .bind(&res.name)
        .bind(&now)
        .bind(&now)
        .execute(self.pool.deref())
        .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(Status::internal(format!("failed to create Group: {}", err)));
            }
        };

        match sqlx::query(
            "INSERT INTO group_members(group_id, user_id, is_admin, created_at) VALUES($1, $2, $3, $4)",
        )
        .bind(&id)
        .bind(&requester_id)
        .bind(&true)
        .bind(&now)
        .execute(self.pool.deref())
        .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(Status::internal(format!("failed to assign user to group: {}", err)));
            }
        };

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::groups::get", skip(self))]
    pub async fn get(&self, claims: &token::Claims, id: &Uuid) -> Result<Group, Status> {
        if !claims.adm && !claims.has_group(&id.to_hyphenated().to_string()) {
            return Err(Status::permission_denied(
                "you are not allowed to see this group",
            ));
        }
        let row: GroupRow = match sqlx::query_as(
            "SELECT id, name, created_at, updated_at FROM groups WHERE id = $1",
        )
        .bind(id)
        .fetch_one(self.pool.deref())
        .await
        {
            Ok(row) => row,
            Err(err) => {
                return Err(Status::not_found(format!("failed to find group: {}", err)));
            }
        };

        let res = Group {
            id: row.id.to_hyphenated().to_string(),
            name: row.name,
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

    #[tracing::instrument(name = "mgr::groups::delete", skip(self))]
    pub async fn delete(&self, claims: &token::Claims, id: &Uuid) -> Result<Group, Status> {
        if !claims.adm {
            let row = match sqlx::query(
                "SELECT is_admin FROM group_members WHERE group_id = $1 AND user_id = $2",
            )
            .bind(id)
            .bind(&claims.sub)
            .fetch_one(self.pool.deref())
            .await
            {
                Ok(val) => val,
                Err(err) => {
                    return Err(Status::permission_denied(format!(
                        "you are not a member of this group: {}",
                        err
                    )))
                }
            };
            let is_admin: bool = row.get(0);
            if !is_admin {
                return Err(Status::permission_denied(
                    "you are not a admin of this group",
                ));
            }
        }
        let res = self.get(claims, id).await?;

        match sqlx::query("DELETE FROM groups WHERE id = $1")
            .bind(&id)
            .execute(self.pool.deref())
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(Status::internal(format!("failed to delete Group: {}", err)));
            }
        };

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::groups::update", skip(self))]
    pub async fn update(
        &self,
        claims: &token::Claims,
        id: &Uuid,
        name: &String,
    ) -> Result<Group, Status> {
        if !claims.adm {
            let row = match sqlx::query(
                "SELECT is_admin FROM group_members WHERE group_id = $1 AND user_id = $2",
            )
            .bind(id)
            .bind(&claims.sub)
            .fetch_one(self.pool.deref())
            .await
            {
                Ok(val) => val,
                Err(err) => {
                    return Err(Status::permission_denied(format!(
                        "you are not a member of this group: {}",
                        err
                    )))
                }
            };
            let is_admin: bool = row.get(0);
            if !is_admin {
                return Err(Status::permission_denied(
                    "you are not a admin of this group",
                ));
            }
        }
        let mut group = self.get(claims, id).await?;

        let now = chrono::Utc::now();

        match sqlx::query("UPDATE groups SET name = $1, updated_at = $2 WHERE id = $3")
            .bind(&name)
            .bind(&now)
            .bind(id)
            .execute(self.pool.deref())
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(Status::internal(format!("failed to update Group: {}", err)));
            }
        }

        group.name = name.clone();
        group.updated_at = Some(prost_types::Timestamp {
            seconds: now.timestamp(),
            nanos: 0,
        });

        Ok(group)
    }

    #[tracing::instrument(name = "mgr::groups::add_user", skip(self))]
    pub async fn add_user(
        &self,
        claims: &token::Claims,
        user_id: &Uuid,
        group_id: &Uuid,
        is_admin: &bool,
    ) -> Result<(), Status> {
        if !claims.adm {
            let requester_id = match Uuid::parse_str(&claims.sub) {
                Ok(id) => id,
                Err(err) => {
                    return Err(Status::permission_denied(format!(
                        "failed to parse user id from token: {}",
                        err
                    )))
                }
            };
            let row = match sqlx::query(
                "SELECT is_admin FROM group_members WHERE group_id = $1 AND user_id = $2",
            )
            .bind(group_id)
            .bind(&requester_id)
            .fetch_one(self.pool.deref())
            .await
            {
                Ok(val) => val,
                Err(err) => {
                    return Err(Status::permission_denied(format!(
                        "you are not a member of this group: {}",
                        err
                    )))
                }
            };
            let is_admin: bool = row.get(0);
            if !is_admin {
                return Err(Status::permission_denied(
                    "you are not a admin of this group",
                ));
            }
        }

        let now = chrono::Utc::now();
        match sqlx::query("INSERT INTO group_members(group_id, user_id, is_admin, created_at) VALUES($1, $2, $3, $4)").
            bind(group_id).
            bind(user_id).
            bind(is_admin).
            bind(&now).
            execute(self.pool.deref()).await {
            Ok(_) => {},
            Err(err) => {
                return Err(Status::invalid_argument(format!("can't assign user to group: {}", err)));
            },
        };

        Ok(())
    }

    #[tracing::instrument(name = "mgr::groups::remove_user", skip(self))]
    pub async fn remove_user(
        &self,
        claims: &token::Claims,
        user_id: &Uuid,
        group_id: &Uuid,
    ) -> Result<(), Status> {
        if !claims.adm {
            let requester_id = match Uuid::parse_str(&claims.sub) {
                Ok(id) => id,
                Err(err) => {
                    return Err(Status::permission_denied(format!(
                        "failed to parse user id from token: {}",
                        err
                    )))
                }
            };
            let row = match sqlx::query(
                "SELECT is_admin FROM group_members WHERE group_id = $1 AND user_id = $2",
            )
            .bind(group_id)
            .bind(&requester_id)
            .fetch_one(self.pool.deref())
            .await
            {
                Ok(val) => val,
                Err(err) => {
                    return Err(Status::permission_denied(format!(
                        "you are not a member of this group: {}",
                        err
                    )))
                }
            };
            let is_admin: bool = row.get(0);
            if !is_admin {
                return Err(Status::permission_denied(
                    "you are not a admin of this group",
                ));
            }
        }

        match sqlx::query("DELETE group_members WHERE group_id = $1 AND user_id = $2")
            .bind(group_id)
            .bind(user_id)
            .execute(self.pool.deref())
            .await
        {
            Ok(_) => {}
            Err(err) => {
                return Err(Status::invalid_argument(format!(
                    "can't remove user from group: {}",
                    err
                )));
            }
        };
        Ok(())
    }

    #[tracing::instrument(name = "mgr::groups::list", skip(self))]
    pub async fn list(
        &self,
        claims: &token::Claims,
    ) -> Result<
        Pin<Box<impl Stream<Item = Result<Group, tonic::Status>> + Send + Sync + 'static>>,
        Status,
    > {
        let (tx, rx) = mpsc::channel(4);

        let pool = self.pool.clone();
        let is_admin = claims.adm;
        let user_id = match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(err) => {
                return Err(Status::permission_denied(format!(
                    "failed to parse user id from token: {}",
                    err
                )))
            }
        };

        tokio::spawn(async move {
            let mut rows = match is_admin {
                true => sqlx::query_as("SELECT id, name, created_at, updated_at FROM groups")
                    .fetch(pool.deref()),
                false => sqlx::query_as(
                    r#"
                    SELECT id, name, groups.created_at, groups.updated_at 
                    FROM groups LEFT JOIN group_members ON (groups.id = group_members.group_id) 
                    WHERE group_members.user_id = $1
                    "#,
                )
                .bind(&user_id)
                .fetch(pool.deref()),
            };
            loop {
                let row: GroupRow = match rows.try_next().await {
                    Ok(row) => match row {
                        Some(row) => row,
                        None => {
                            break;
                        }
                    },
                    Err(err) => {
                        return Err(Status::internal(format!("failed to update Group: {}", err)));
                    }
                };
                let res = Group {
                    id: row.id.to_hyphenated().to_string(),
                    name: row.name,
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
                        return Err(Status::internal(format!("failed to send Group: {}", err)));
                    }
                };
            }
            Ok(())
        });

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    #[tracing::instrument(name = "mgr::groups::list_as_vector", skip(self))]
    pub async fn list_as_vector(&self, user_id: &Uuid) -> Result<Vec<Group>, Status> {
        let mut rows = sqlx::query_as(
            r#"
            SELECT id, name, groups.created_at, groups.updated_at 
            FROM groups LEFT JOIN group_members ON (groups.id = group_members.group_id) 
            WHERE group_members.user_id = $1
            "#,
        )
        .bind(&user_id)
        .fetch(self.pool.deref());

        let mut res = vec![];

        loop {
            let row: GroupRow = match rows.try_next().await {
                Ok(row) => match row {
                    Some(row) => row,
                    None => {
                        break;
                    }
                },
                Err(err) => {
                    return Err(Status::internal(format!("failed to update Group: {}", err)));
                }
            };
            let grp = Group {
                id: row.id.to_hyphenated().to_string(),
                name: row.name,
                created_at: Some(prost_types::Timestamp {
                    seconds: row.created_at.timestamp(),
                    nanos: 0,
                }),
                updated_at: Some(prost_types::Timestamp {
                    seconds: row.updated_at.timestamp(),
                    nanos: 0,
                }),
            };
            res.push(grp);
        }

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::groups::list_members", skip(self))]
    pub async fn list_members(
        &self,
        claims: &token::Claims,
        group_id: &Uuid,
    ) -> Result<
        Pin<Box<impl Stream<Item = Result<GroupMember, Status>> + Send + Sync + 'static>>,
        Status,
    > {
        let requester_id = match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(err) => {
                return Err(Status::permission_denied(format!(
                    "failed to parse user id from token: {}",
                    err
                )))
            }
        };
        match sqlx::query("SELECT is_admin FROM group_members WHERE group_id = $1 AND user_id = $2")
            .bind(group_id)
            .bind(&requester_id)
            .fetch_one(self.pool.deref())
            .await
        {
            Ok(_) => {}
            Err(err) => {
                return Err(Status::permission_denied(format!(
                    "you are not a member of this group: {}",
                    err
                )))
            }
        };

        let (tx, rx) = mpsc::channel(4);

        let pool = self.pool.clone();
        let group_id = group_id.clone();

        tokio::spawn(async move {
            let mut rows = sqlx::query_as("SELECT id, name, email, group_members.is_admin, group_members.created_at AS joined_at FROM users LEFT JOIN group_members ON(users.id = group_members.user_id) WHERE group_members.group_id = $1")
                .bind(group_id)
                .fetch(pool.deref());
            loop {
                let row: GroupMemberRow = match rows.try_next().await {
                    Ok(row) => match row {
                        Some(row) => row,
                        None => {
                            break;
                        }
                    },
                    Err(err) => {
                        return Err(Status::internal(format!("failed to update Group: {}", err)));
                    }
                };
                let res = GroupMember {
                    user_id: row.id.to_hyphenated().to_string(),
                    user_name: row.name,
                    user_email: row.email,
                    is_admin: row.is_admin,
                    joined_at: Some(prost_types::Timestamp {
                        seconds: row.joined_at.timestamp(),
                        nanos: 0,
                    }),
                };
                match tx.send(Ok(res)).await {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(Status::internal(format!("failed to send Group: {}", err)));
                    }
                };
            }
            Ok(())
        });

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
