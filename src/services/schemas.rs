use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api;
use crate::token;
use api::catalog::schemas_server::Schemas;
use api::catalog::{
    CreateSchemaRequest, DeleteSchemaRequest, GetSchemaRequest, ListSchemasRequest, Schema,
    UpdateSchemaRequest,
};

use crate::managers;

use super::base::BaseService;

#[derive(Debug)]
pub struct Service {
    mgr: Arc<managers::schemas::Manager>,
    validator: Arc<token::Validator>,
}

impl BaseService for Service {}

impl Service {
    pub fn new(
        mgr: Arc<managers::schemas::Manager>,
        validator: Arc<token::Validator>,
    ) -> Result<Service, sqlx::Error> {
        let res = Service {
            mgr,
            validator,
        };
        Ok(res)
    }

    #[tracing::instrument(skip(self))]
    fn check_admin<T: std::fmt::Debug>(&self, req: &Request<T>) -> Result<(), Status> {
        let claims = self.validator.get_access_token_claims(req)?;
        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins are allowed to access the schema service",
            ));
        }
        Ok(())
    }
}

#[tonic::async_trait]
impl Schemas for Service {
    #[tracing::instrument(name = "svc::schemas::create", skip(self))]
    async fn create(
        &self,
        request: Request<CreateSchemaRequest>,
    ) -> Result<Response<Schema>, Status> {
        self.check_admin(&request)?;
        let data = Self::parse_json(&request.get_ref().data)?;
        let res = self.mgr.create(&request.get_ref().kind, &data).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::schemas::get", skip(self))]
    async fn get(&self, request: Request<GetSchemaRequest>) -> Result<Response<Schema>, Status> {
        self.check_admin(&request)?;
        let id = Self::parse_uuid(&request.get_ref().id)?;
        let res = self.mgr.get(&id).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::schemas::delete", skip(self))]
    async fn delete(
        &self,
        request: Request<DeleteSchemaRequest>,
    ) -> Result<Response<Schema>, Status> {
        self.check_admin(&request)?;
        let id = Self::parse_uuid(&request.get_ref().id)?;
        let res = self.mgr.delete(&id).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::schemas::update", skip(self))]
    async fn update(
        &self,
        request: Request<UpdateSchemaRequest>,
    ) -> Result<Response<Schema>, Status> {
        self.check_admin(&request)?;
        let id = Self::parse_uuid(&request.get_ref().id)?;
        let patch_object = Self::parse_json(&request.get_ref().data)?;
        let result = self.mgr.update(&id, &patch_object).await?;
        Ok(Response::new(result))
    }

    type ListStream = Pin<Box<dyn Stream<Item = Result<Schema, Status>> + Send + Sync + 'static>>;

    #[tracing::instrument(name = "svc::schemas::list", skip(self))]
    async fn list(
        &self,
        request: Request<ListSchemasRequest>,
    ) -> Result<Response<Self::ListStream>, Status> {
        self.check_admin(&request)?;
        let res = self.mgr.list().await?;
        Ok(Response::new(res))
    }
}
