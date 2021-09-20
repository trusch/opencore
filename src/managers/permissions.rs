use super::errors::Error;
use crate::api;
use crate::token::Claims;

use futures::{Stream, TryStreamExt};
use std::pin::Pin;

use api::catalog::PermissionInfo;
use log::info;
use sqlx::types::Uuid;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::Status;

#[derive(Debug)]
pub struct Manager {
    pool: Arc<sqlx::PgPool>,
}

#[derive(sqlx::FromRow)]
struct PermissionInfoRow {
    resource_id: Uuid,
    principal_id: Uuid,
    actions: Vec<String>,
}

impl Manager {
    pub async fn new(pool: Arc<sqlx::PgPool>) -> Result<Manager, Error> {
        let res = Manager { pool };
        res.init_tables().await?;
        Ok(res)
    }

    #[tracing::instrument(skip(self))]
    async fn init_tables(&self) -> Result<(), Error> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS
                permissions(
                    resource_id UUID,
                    principal_id UUID,
                    action TEXT,
                    PRIMARY KEY(resource_id, principal_id, action)
                );"#,
        )
        .execute(self.pool.deref())
        .await?;
        sqlx::query(
            r#"CREATE INDEX IF NOT EXISTS permissions_principal_id_idx ON permissions(principal_id);"#,
        )
        .execute(self.pool.deref())
        .await?;
        sqlx::query(
            r#"CREATE INDEX IF NOT EXISTS permissions_resource_id_idx ON permissions(resource_id);"#,
        )
        .execute(self.pool.deref())
        .await?;
        Ok(())
    }

    #[tracing::instrument(name = "mgr::permissions::share", skip(self))]
    pub async fn share(
        &self,
        claims: &Claims,
        resource_id: &Uuid,
        principal_id: &Uuid,
        actions: &[String],
    ) -> Result<PermissionInfo, Error> {
        self.check(resource_id, "grant", claims).await?;

        for action in actions.iter() {
            sqlx::query(
                r#"INSERT INTO 
                    permissions(resource_id, principal_id, action) 
                    VALUES($1, $2, $3)"#,
            )
            .bind(resource_id)
            .bind(principal_id)
            .bind(&action)
            .execute(self.pool.deref())
            .await?;
        }

        self.get(claims, resource_id, principal_id).await
    }

    #[tracing::instrument(name = "mgr::permissions::share_with_tx", skip(self))]
    pub async fn share_with_tx<'c>(
        &self,
        tx: &mut sqlx::Transaction<'c, sqlx::Postgres>,
        claims: &Claims,
        resource_id: &Uuid,
        principal_id: &Uuid,
        actions: &[String],
    ) -> Result<PermissionInfo, Error> {
        self.check_with_tx(tx, resource_id, "grant", claims).await?;

        for action in actions.iter() {
            sqlx::query(
                r#"INSERT INTO 
                    permissions(resource_id, principal_id, action) 
                    VALUES($1, $2, $3)"#,
            )
            .bind(resource_id)
            .bind(principal_id)
            .bind(&action)
            .execute(&mut *tx)
            .await?;
        }

        self.get_with_tx(tx, claims, resource_id, principal_id)
            .await
    }

    #[tracing::instrument(name = "mgr::permissions::unshare", skip(self))]
    pub async fn unshare(
        &self,
        claims: &Claims,
        resource_id: &Uuid,
        principal_id: &Uuid,
        actions: &[String],
    ) -> Result<PermissionInfo, Error> {
        self.check(resource_id, "grant", claims).await?;

        sqlx::query(
            r#"DELETE FROM permissions 
                WHERE resource_id = $1 
                AND principal_id = $2 
                AND action IN $3"#,
        )
        .bind(resource_id)
        .bind(principal_id)
        .bind(actions)
        .execute(self.pool.deref())
        .await?;

        self.get(claims, resource_id, principal_id).await
    }

    #[tracing::instrument(name = "mgr::permissions::get", skip(self))]
    pub async fn get(
        &self,
        claims: &Claims,
        resource_id: &Uuid,
        principal_id: &Uuid,
    ) -> Result<PermissionInfo, Error> {
        self.check(resource_id, "read", claims).await?;

        let row: PermissionInfoRow = sqlx::query_as(
            r#"SELECT resource_id, principal_id, array_agg(action) AS actions
                FROM permissions 
                WHERE resource_id = $1 
                AND principal_id = $2
                GROUP BY resource_id, principal_id"#,
        )
        .bind(resource_id)
        .bind(principal_id)
        .fetch_one(self.pool.deref())
        .await?;
        Ok(PermissionInfo {
            resource_id: row.resource_id.to_hyphenated().to_string(),
            principal_id: row.principal_id.to_hyphenated().to_string(),
            actions: row.actions,
        })
    }

    #[tracing::instrument(name = "mgr::permissions::get_with_tx", skip(self))]
    pub async fn get_with_tx<'c>(
        &self,
        tx: &mut sqlx::Transaction<'c, sqlx::Postgres>,
        claims: &Claims,
        resource_id: &Uuid,
        principal_id: &Uuid,
    ) -> Result<PermissionInfo, Error> {
        self.check(resource_id, "read", claims).await?;

        let row: PermissionInfoRow = sqlx::query_as(
            r#"
            SELECT resource_id, principal_id, array_agg(action) AS actions
            FROM permissions 
            WHERE resource_id = $1 
            AND principal_id = $2
            GROUP BY resource_id, principal_id"#,
        )
        .bind(resource_id)
        .bind(principal_id)
        .fetch_one(tx)
        .await?;
        Ok(PermissionInfo {
            resource_id: row.resource_id.to_hyphenated().to_string(),
            principal_id: row.principal_id.to_hyphenated().to_string(),
            actions: row.actions,
        })
    }

    #[tracing::instrument(name = "mgr::permissions::check", skip(self))]
    pub async fn check(
        &self,
        resource_id: &Uuid,
        action: &str,
        claims: &Claims,
    ) -> Result<(), Error> {
        info!(
            "permission check: resource_id: {}, action: {}, claims: {:?}",
            resource_id, action, claims
        );

        if claims.adm {
            return Ok(());
        }
        let row: (i64,) = sqlx::query_as(
            r#"SELECT count(*)
                FROM resources
                LEFT JOIN permissions ON (resources.permission_parent_id = permissions.resource_id) 
                WHERE resources.resource_id = $1 
                AND principal_id = ANY($2)
                AND action = $3"#,
        )
        .bind(resource_id)
        .bind(&claims.principals()?)
        .bind(action)
        .fetch_one(self.pool.deref())
        .await?;

        match row.0 {
            0 => Err(Error::Forbidden),
            _ => Ok(()),
        }
    }

    #[tracing::instrument(name = "mgr::permissions::check_with_tx", skip(self))]
    pub async fn check_with_tx<'r>(
        &self,
        tx: &mut sqlx::Transaction<'r, sqlx::Postgres>,
        resource_id: &Uuid,
        action: &str,
        claims: &Claims,
    ) -> Result<(), Error> {
        if claims.adm {
            return Ok(());
        }
        let row: (i64,) = sqlx::query_as(
            r#"SELECT count(*) 
                FROM resources
                LEFT JOIN permissions ON (resources.permission_parent_id = permissions.resource_id)
                WHERE resource_id = $1 
                AND principal_id IN $2
                AND action = $3"#,
        )
        .bind(resource_id)
        .bind(&claims.principals()?)
        .bind(action)
        .fetch_one(tx)
        .await?;

        match row.0 {
            0 => Err(Error::Forbidden),
            _ => Ok(()),
        }
    }

    #[tracing::instrument(name = "mgr::permissions::check_with_group_resolution", skip(self))]
    pub async fn check_with_group_resolution(
        &self,
        claims: &Claims,
        resource_id: &Uuid,
        user_id: &Uuid,
        action: &str,
    ) -> Result<(), Error> {
        self.check(resource_id, "read", claims).await?;

        info!("check with group resolution");
        info!(
            "rid: {:?}, uid: {:?}, action: {}",
            resource_id, user_id, action
        );

        let row: (bool,) = sqlx::query_as(
            r#"
                WITH user_is_allowed AS (
                    SELECT count(*)>0 AS allowed FROM permissions WHERE resource_id = $1 AND principal_id = $2 AND action = $3
                ),
                group_is_allowed AS (
                    SELECT count(*)>0 AS allowed FROM permissions WHERE resource_id = $1 AND principal_id = ANY(
                        SELECT group_id FROM group_members WHERE user_id = $2
                    ) AND action = $3)
                SELECT user_is_allowed.allowed OR group_is_allowed.allowed AS allowed FROM user_is_allowed, group_is_allowed; 
                "#,        
        )
        .bind(resource_id)
        .bind(user_id)
        .bind(action)
        .fetch_one(self.pool.deref())
        .await?;

        match row.0 {
            false => Err(Error::Forbidden),
            _ => Ok(()),
        }
    }

    #[tracing::instrument(name = "mgr::permissions::list", skip(self))]
    pub async fn list(
        &self,
        claims: &Claims,
        resource_id: &Uuid,
    ) -> Result<
        Pin<Box<impl Stream<Item = Result<PermissionInfo, Status>> + Send + Sync + 'static>>,
        Status,
    > {
        log::info!("call list permissions");
        self.check(resource_id, "grant", claims).await?;
        log::info!("list call authorized");
        let (tx, rx) = mpsc::channel(4);
        let pool = self.pool.clone();
        let resource_id = *resource_id;
        tokio::spawn(async move {
            let mut rows = sqlx::query_as(
                r#"SELECT resources.resource_id, principal_id, array_agg(action) as actions
                    FROM resources
                    LEFT JOIN permissions ON (resources.permission_parent_id = permissions.resource_id)
                    WHERE resources.resource_id = $1
                    GROUP BY principal_id, resources.resource_id"#,
            )
            .bind(resource_id)
            .fetch(pool.deref());
            loop {
                let row: PermissionInfoRow = match rows.try_next().await {
                    Ok(row) => match row {
                        Some(row) => row,
                        None => {
                            break;
                        }
                    },
                    Err(err) => {
                        log::error!("failed to read permission row: {}", err);
                        break;
                    }
                };
                let res = PermissionInfo {
                    resource_id: row.resource_id.to_hyphenated().to_string(),
                    principal_id: row.principal_id.to_hyphenated().to_string(),
                    actions: row.actions,
                };
                match tx.send(Ok(res)).await {
                    Ok(_) => (),
                    Err(err) => {
                        log::error!("failed to send permission info: {}", err);
                        break;
                    }
                };
                log::debug!("send out permission info stream item");
            }
        });

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
