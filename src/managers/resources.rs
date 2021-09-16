use super::errors::Error;

use futures::{Stream, TryStreamExt};
use sqlx::types::Uuid;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::Instrument;

use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
sea_query::sea_query_driver_postgres!();
use sea_query_driver_postgres::bind_query_as;

use crate::api;
use crate::managers;
use crate::token::Claims;

use api::catalog::{Resource, ShareRequest};

#[derive(sqlx::FromRow)]
struct ResourceRow {
    resource_id: Uuid,
    kind: String,
    parent_id: Option<Uuid>,
    permission_parent_id: Option<Uuid>,
    creator_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    data: serde_json::Value,
    labels: serde_json::Value,
}

impl Into<Resource> for ResourceRow {
    fn into(self) -> Resource {
        Resource {
            id: self.resource_id.to_hyphenated().to_string(),
            kind: self.kind,
            parent_id: self
                .parent_id
                .unwrap_or_default()
                .to_hyphenated()
                .to_string(),
            permission_parent_id: self
                .permission_parent_id
                .unwrap_or_default()
                .to_hyphenated()
                .to_string(),
            creator_id: self.creator_id.to_hyphenated().to_string(),
            labels: serde_json::from_value(self.labels).unwrap_or_default(),
            created_at: Some(prost_types::Timestamp {
                seconds: self.created_at.timestamp(),
                nanos: 0,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: self.updated_at.timestamp(),
                nanos: 0,
            }),
            data: serde_json::to_string(&self.data).unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct Manager {
    pool: Arc<sqlx::PgPool>,
    permissions: Arc<managers::permissions::Manager>,
    schemas: Arc<managers::schemas::Manager>,
    events: Arc<managers::events::Manager>,
}

impl Manager {
    pub async fn new(
        pool: Arc<sqlx::PgPool>,
        permissions: Arc<managers::permissions::Manager>,
        schemas: Arc<managers::schemas::Manager>,
        events: Arc<managers::events::Manager>,
    ) -> Result<Manager, Error> {
        let res = Manager {
            pool: pool,
            permissions: permissions,
            schemas: schemas,
            events: events,
        };
        res.init_tables().await?;
        Ok(res)
    }

    #[tracing::instrument(name = "mgr::resources::init_tables", skip(self))]
    async fn init_tables(&self) -> Result<(), Error> {
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS
            resources(
                resource_id UUID PRIMARY KEY,
                kind TEXT NOT NULL,
                parent_id UUID REFERENCES resources(resource_id) ON DELETE CASCADE,
                permission_parent_id UUID REFERENCES resources(resource_id), 
                creator_id UUID NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                data JSONB,
                labels JSONB,
                data_vec TSVECTOR GENERATED ALWAYS AS (jsonb_to_tsvector('english', data, '["string"]')) STORED
            );"#,
        )
        .execute(self.pool.deref())
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS resources_data_gin_idx ON resources USING GIN (data);",
        )
        .execute(self.pool.deref())
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS resources_data_vec_gin_idx ON resources USING GIN (data_vec);",
        )
        .execute(self.pool.deref())
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS resources_labels_gin_idx ON resources USING GIN (labels);",
        )
        .execute(self.pool.deref())
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS resources_permission_parent_id_idx ON resources(permission_parent_id);",
        )
        .execute(self.pool.deref())
        .await?;

        Ok(())
    }

    #[tracing::instrument(name = "mgr::resources::create", skip(self))]
    pub async fn create(
        &self,
        claims: &Claims,
        kind: &String,
        parent_id: Option<&Uuid>,
        permission_parent_id: Option<&Uuid>,
        data: &serde_json::Value,
        labels: &HashMap<String, String>,
        shares: &Vec<ShareRequest>,
    ) -> Result<Resource, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self
            .create_with_tx(
                claims,
                kind,
                parent_id,
                permission_parent_id,
                data,
                labels,
                shares,
                &mut tx,
            )
            .await?;

        tx.commit().await?;

        self.events
            .publish(
                &Claims::admin(),
                &Uuid::parse_str(&res.id)?,
                &res.kind,
                &res.labels,
                api::catalog::EventType::Create,
                &data,
            )
            .await?;

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::resources::create_with_tx", skip(self))]
    pub async fn create_with_tx(
        &self,
        claims: &Claims,
        kind: &String,
        parent_id: Option<&Uuid>,
        permission_parent_id: Option<&Uuid>,
        data: &serde_json::Value,
        labels: &HashMap<String, String>,
        shares: &Vec<ShareRequest>,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<Resource, Error> {
        self.schemas.validate(kind, data).await?;

        let resource_id = Uuid::new_v4();

        let now = chrono::Utc::now();

        let data_str = serde_json::to_string(data)?;

        let res = Resource {
            id: resource_id.to_hyphenated().to_string(),
            parent_id: parent_id
                .unwrap_or(&resource_id)
                .to_hyphenated()
                .to_string(),
            permission_parent_id: permission_parent_id
                .unwrap_or(&resource_id)
                .to_hyphenated()
                .to_string(),
            creator_id: claims.sub.clone(),
            kind: kind.clone(),
            data: data_str,
            labels: labels.clone(),
            created_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
            updated_at: Some(prost_types::Timestamp {
                seconds: now.timestamp(),
                nanos: 0,
            }),
        };

        let label_value = serde_json::to_value(labels)?;

        sqlx::query("INSERT INTO resources(resource_id, kind, parent_id, permission_parent_id, creator_id, created_at, updated_at, data, labels) VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)").
            bind(&resource_id).
            bind(&res.kind).
            bind(parent_id.unwrap_or(&resource_id)).
            bind(permission_parent_id.unwrap_or(&resource_id)).
            bind(Uuid::parse_str(&res.creator_id)?).
            bind(now).
            bind(now).
            bind(&data).
            bind(&label_value).
            execute(&mut *tx).await?;

        match permission_parent_id {
            Some(_) => {}
            None => {
                let user_id = Uuid::parse_str(&claims.sub)?;
                let perms: Vec<String> = vec!["grant", "read", "write"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect();
                self.permissions
                    .share_with_tx(&mut *tx, &Claims::admin(), &resource_id, &user_id, &perms)
                    .await?;
            }
        }

        for share in shares.iter() {
            let principal_id = match Uuid::parse_str(&share.principal_id) {
                Ok(val) => val,
                Err(_) => Uuid::new_v5(&Uuid::NAMESPACE_OID, &share.principal_id.as_bytes()), // service accounts may be specified by name
            };
            self.permissions
                .share_with_tx(
                    tx,
                    &Claims::admin(),
                    &resource_id,
                    &principal_id,
                    &share.actions,
                )
                .await?;
        }

        Ok(res)
    }

    #[tracing::instrument(name = "mgr::resources::get", skip(self))]
    pub async fn get(&self, claims: &Claims, id: &Uuid) -> Result<Resource, Error> {
        self.permissions.check(id, "read", claims).await?;

        let row: ResourceRow = sqlx::query_as(
            "SELECT resource_id, kind, parent_id, permission_parent_id, creator_id, created_at, updated_at, data, labels FROM resources WHERE resource_id = $1",
        )
        .bind(id)
        .fetch_one(self.pool.deref())
        .await?;

        log::info!("got row");

        Ok(row.into())
    }

    #[tracing::instrument(name = "mgr::resources::delete", skip(self))]
    pub async fn delete(&self, claims: &Claims, id: &Uuid) -> Result<Resource, Error> {
        self.permissions.check(id, "write", claims).await?;

        let res = self.get(claims, id).await?;

        sqlx::query("DELETE FROM resources WHERE resource_id = $1")
            .bind(&id)
            .execute(self.pool.deref())
            .await?;

        self.events
            .publish(
                &Claims::admin(),
                &id,
                &res.kind,
                &res.labels,
                api::catalog::EventType::Delete,
                &serde_json::from_str(&res.data)?,
            )
            .await?;
        Ok(res)
    }

    #[tracing::instrument(name = "mgr::resources::update", skip(self))]
    pub async fn update(
        &self,
        claims: &Claims,
        id: &Uuid,
        doc: &serde_json::Value,
        labels: &HashMap<String, String>,
    ) -> Result<Resource, Error> {
        let mut tx = self.pool.begin().await?;

        self.permissions.check(id, "write", claims).await?;

        let mut resource = self.get(claims, id).await?;

        let mut data = serde_json::from_str(&resource.data)?;

        let now = chrono::Utc::now();

        json_patch::merge(&mut data, doc);

        self.schemas.validate(&resource.kind, &data).await?;

        let mut final_labels = resource.labels.clone();
        if labels.len() > 0 {
            // empty string marks label for removal
            for (k, v) in labels.iter() {
                if v.len() == 0 {
                    final_labels.remove(k);
                } else {
                    final_labels.insert(k.to_string(), v.to_string());
                }
            }
            sqlx::query("UPDATE resources SET data = $1, updated_at = $2, labels = $3 WHERE resource_id = $4")
                .bind(&data)
                .bind(&now)
                .bind(serde_json::to_value(&final_labels)?)
                .bind(id)
                .execute(&mut tx)
                .await?;
        } else {
            sqlx::query("UPDATE resources SET data = $1, updated_at = $2 WHERE resource_id = $3")
                .bind(&data)
                .bind(&now)
                .bind(id)
                .execute(&mut tx)
                .await?;
        }

        resource.data = serde_json::to_string(&data)?;
        resource.labels = final_labels;
        resource.updated_at = Some(prost_types::Timestamp {
            seconds: now.timestamp(),
            nanos: 0,
        });

        tx.commit().await?;

        self.events
            .publish(
                &Claims::admin(),
                &id,
                &resource.kind,
                &resource.labels,
                api::catalog::EventType::Update,
                &data,
            )
            .await?;

        Ok(resource)
    }

    #[tracing::instrument(name = "mgr::resources::list", skip(self))]
    pub async fn list(
        &self,
        claims: &Claims,
        labels: &HashMap<String, String>,
        filter: &String,
        kind: &String,
        search_term: &String,
    ) -> Result<
        Pin<Box<impl Stream<Item = Result<Resource, tonic::Status>> + Send + Sync + 'static>>,
        tonic::Status,
    > {
        let (tx, rx) = mpsc::channel(4);
        let mut transaction = match self.pool.begin().await {
            Ok(val) => val,
            Err(err) => {
                return Err(tonic::Status::internal(format!(
                    "failed to start transaction: {}",
                    err
                )));
            }
        };

        let mut query = Query::select();

        let span = tracing::debug_span!("prepare");

        span.in_scope(|| {
            let principals = claims.principals()?;

            query = query
                .expr(sea_query::Expr::cust(
                    "DISTINCT ON(resources.resource_id) resources.resource_id",
                ))
                .columns(vec![
                    (ResourcesTable::Table, ResourcesTable::Kind),
                    (ResourcesTable::Table, ResourcesTable::ParentID),
                    (ResourcesTable::Table, ResourcesTable::PermissionParentID),
                    (ResourcesTable::Table, ResourcesTable::CreatorID),
                    (ResourcesTable::Table, ResourcesTable::CreatedAt),
                    (ResourcesTable::Table, ResourcesTable::UpdatedAt),
                    (ResourcesTable::Table, ResourcesTable::Data),
                    (ResourcesTable::Table, ResourcesTable::Labels),
                ])
                .from(ResourcesTable::Table)
                .join(
                    sea_query::JoinType::LeftJoin,
                    PermissionsTable::Table,
                    Expr::tbl(ResourcesTable::Table, ResourcesTable::PermissionParentID)
                        .equals(PermissionsTable::Table, PermissionsTable::ResourceID),
                )
                .order_by(
                    (ResourcesTable::Table, ResourcesTable::ResourceID),
                    sea_query::Order::Desc,
                )
                .order_by(
                    (ResourcesTable::Table, ResourcesTable::CreatedAt),
                    sea_query::Order::Desc,
                )
                .to_owned();

            if !claims.adm {
                query = query
                    .and_where(
                        Expr::tbl(PermissionsTable::Table, PermissionsTable::PrincipalID)
                            .is_in(principals.clone()),
                    )
                    .and_where(
                        Expr::tbl(PermissionsTable::Table, PermissionsTable::Action).eq("read"),
                    )
                    .to_owned();
            }

            if labels.len() > 0 {
                let labels_value = match serde_json::to_value(&labels) {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(tonic::Status::internal(format!(
                            "failed to encode resource labels: {}",
                            err
                        )));
                    }
                };
                query = query
                    .and_where(sea_query::Expr::cust_with_values(
                        "labels @> ?",
                        vec![labels_value],
                    ))
                    .to_owned();
            }

            if filter != "" {
                query = query
                    .and_where(Expr::cust_with_values(
                        "data @@ (?::JSONPATH)",
                        vec![filter.as_str()],
                    ))
                    .to_owned();
            }

            if kind != "" {
                query = query
                    .and_where(
                        Expr::tbl(ResourcesTable::Table, ResourcesTable::Kind).eq(kind.as_str()),
                    )
                    .to_owned();
            }

            if search_term != "" {
                query = query
                    .and_where(Expr::cust_with_values(
                        "data_vec @@ (websearch_to_tsquery(?))",
                        vec![search_term.as_str()],
                    ))
                    .to_owned();
            }

            query = Query::select()
                .expr(sea_query::Expr::cust("*"))
                .from_subquery(query.clone(), ResourcesTable::SubQuery)
                .order_by(
                    (ResourcesTable::SubQuery, ResourcesTable::CreatedAt),
                    sea_query::Order::Desc,
                )
                .to_owned();

            Ok(())
        })?;

        let span = tracing::info_span!("fetch_rows");
        let (sql, values) = query.build(PostgresQueryBuilder);
        tokio::spawn(
            async move {
                let mut rows = bind_query_as(sqlx::query_as::<_, ResourceRow>(&sql), &values)
                    .fetch(&mut transaction);
                loop {
                    let row: ResourceRow = match rows
                        .try_next()
                        .instrument(tracing::debug_span!("fetch_row"))
                        .await
                    {
                        Ok(row) => match row {
                            Some(row) => row,
                            None => {
                                break;
                            }
                        },
                        Err(err) => {
                            log::error!("failed to parse row: {:?}", err);
                            return Err(());
                        }
                    };

                    let res: Resource = row.into();

                    match tx
                        .send(Ok(res))
                        .instrument(tracing::debug_span!("send_resource"))
                        .await
                    {
                        Ok(_) => (),
                        Err(_) => {
                            return Err(());
                        }
                    };
                }
                drop(rows);
                match transaction
                    .commit()
                    .instrument(tracing::debug_span!("close transaction"))
                    .await
                {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(_) => {
                        return Err(());
                    }
                };
            }
            .instrument(span),
        );

        Ok(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}

pub enum ResourcesTable {
    Table,
    ResourceID,
    Kind,
    ParentID,
    PermissionParentID,
    CreatorID,
    CreatedAt,
    UpdatedAt,
    Data,
    Labels,

    SubQuery,
}

// Mapping between Enum variant and its corresponding string value
impl Iden for ResourcesTable {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "resources",
                Self::ResourceID => "resource_id",
                Self::Kind => "kind",
                Self::ParentID => "parent_id",
                Self::PermissionParentID => "permission_parent_id",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
                Self::Data => "data",
                Self::Labels => "labels",
                Self::CreatorID => "creator_id",
                Self::SubQuery => "sub_query",
            }
        )
        .unwrap();
    }
}

pub enum PermissionsTable {
    Table,
    ResourceID,
    PrincipalID,
    Action,
}

// Mapping between Enum variant and its corresponding string value
impl Iden for PermissionsTable {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "permissions",
                Self::ResourceID => "resource_id",
                Self::PrincipalID => "principal_id",
                Self::Action => "action",
            }
        )
        .unwrap();
    }
}
