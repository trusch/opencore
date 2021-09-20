use futures::Stream;
use sqlx::types::Uuid;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api;
use api::catalog::resources_server::Resources;
use api::catalog::{
    CreateResourceRequest, DeleteResourceRequest, GetResourceRequest, ListResourcesRequest,
    Resource, UpdateResourceRequest,
};

use crate::managers;
use crate::token;

use super::base::BaseService;

#[derive(Debug)]
pub struct Service {
    mgr: Arc<managers::resources::Manager>,
    validator: Arc<token::Validator>,
}

impl BaseService for Service {}

impl Service {
    pub fn new(
        mgr: Arc<managers::resources::Manager>,
        validator: Arc<token::Validator>,
    ) -> Result<Service, sqlx::Error> {
        let res = Service { mgr, validator };
        Ok(res)
    }
}

#[tonic::async_trait]
impl Resources for Service {
    #[tracing::instrument(name = "svc::resources::create", skip(self))]
    async fn create(
        &self,
        request: Request<CreateResourceRequest>,
    ) -> Result<Response<Resource>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let r = request.get_ref();
        let data = Self::parse_json(&r.data)?;

        let parent_id_value: Uuid;
        let mut parent_id: Option<&Uuid> = None;
        if !r.parent_id.is_empty() {
            parent_id_value = Self::parse_uuid(&r.parent_id)?;
            parent_id = Some(&parent_id_value);
        }

        let permission_parent_id_value: Uuid;
        let mut permission_parent_id: Option<&Uuid> = None;
        if !r.permission_parent_id.is_empty() {
            permission_parent_id_value = Self::parse_uuid(&r.permission_parent_id)?;
            permission_parent_id = Some(&permission_parent_id_value);
        }
        let res = self
            .mgr
            .create(crate::managers::resources::CreateOptions {
                claims: &claims,
                kind: &request.get_ref().kind,
                parent_id,
                permission_parent_id,
                data: &data,
                labels: &request.get_ref().labels,
                shares: &request.get_ref().shares,
            })
            .await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::resources::get", skip(self))]
    async fn get(
        &self,
        request: Request<GetResourceRequest>,
    ) -> Result<Response<Resource>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let id = Self::parse_uuid(&request.get_ref().id)?;
        let res = self.mgr.get(&claims, &id).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::resources::delete", skip(self))]
    async fn delete(
        &self,
        request: Request<DeleteResourceRequest>,
    ) -> Result<Response<Resource>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let id = Self::parse_uuid(&request.get_ref().id)?;
        let res = self.mgr.delete(&claims, &id).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::resources::update", skip(self))]
    async fn update(
        &self,
        request: Request<UpdateResourceRequest>,
    ) -> Result<Response<Resource>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let id = Self::parse_uuid(&request.get_ref().id)?;
        let patch_object = Self::parse_json(&request.get_ref().data)?;
        let labels = &request.get_ref().labels;
        let result = self.mgr.update(&claims, &id, &patch_object, labels).await?;
        Ok(Response::new(result))
    }

    type ListStream = Pin<Box<dyn Stream<Item = Result<Resource, Status>> + Send + Sync + 'static>>;

    #[tracing::instrument(name = "svc::resources::list", skip(self))]
    async fn list(
        &self,
        request: Request<ListResourcesRequest>,
    ) -> Result<Response<Self::ListStream>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let res = self
            .mgr
            .list(
                &claims,
                &request.get_ref().labels,
                &request.get_ref().filter,
                &request.get_ref().kind,
                &request.get_ref().query,
            )
            .await?;
        Ok(Response::new(res))
    }
}
