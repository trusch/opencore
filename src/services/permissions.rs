use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api;
use api::catalog::permissions_server::Permissions;
use api::catalog::{
    GetPermissionInfoRequest, ListPermissionsRequest, PermissionCheckRequest,
    PermissionCheckResponse, PermissionInfo, ShareRequest, UnshareRequest,
};

use crate::managers;
use crate::token;

use super::base::BaseService;

#[derive(Debug)]
pub struct Service {
    mgr: Arc<managers::permissions::Manager>,
    validator: Arc<token::Validator>,
}

impl BaseService for Service {}

impl Service {
    pub fn new(
        mgr: Arc<managers::permissions::Manager>,
        validator: Arc<token::Validator>,
    ) -> Result<Service, sqlx::Error> {
        let res = Service { mgr, validator };
        Ok(res)
    }
}

#[tonic::async_trait]
impl Permissions for Service {
    #[tracing::instrument(name = "svc::permissions::share", skip(self))]
    async fn share(
        &self,
        request: Request<ShareRequest>,
    ) -> Result<Response<PermissionInfo>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let resource_id = Self::parse_uuid(&request.get_ref().resource_id)?;
        let principal_id = Self::parse_uuid(&request.get_ref().principal_id)?;
        let res = self
            .mgr
            .share(
                &claims,
                &resource_id,
                &principal_id,
                &request.get_ref().actions,
            )
            .await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::permissions::check", skip(self))]
    async fn check(
        &self,
        request: Request<PermissionCheckRequest>,
    ) -> Result<Response<PermissionCheckResponse>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let resource_id = Self::parse_uuid(&request.get_ref().resource_id)?;
        let principal_id = Self::parse_uuid(&request.get_ref().principal_id)?;
        match self
            .mgr
            .check_with_group_resolution(
                &claims,
                &resource_id,
                &principal_id,
                &request.get_ref().action,
            )
            .await
        {
            Ok(()) => Ok(Response::new(PermissionCheckResponse { granted: true })),
            Err(_) => Ok(Response::new(PermissionCheckResponse { granted: false })),
        }
    }

    #[tracing::instrument(name = "svc::permissions::unshare", skip(self))]
    async fn unshare(
        &self,
        request: Request<UnshareRequest>,
    ) -> Result<Response<PermissionInfo>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let resource_id = Self::parse_uuid(&request.get_ref().resource_id)?;
        let principal_id = Self::parse_uuid(&request.get_ref().principal_id)?;
        let res = self
            .mgr
            .unshare(
                &claims,
                &resource_id,
                &principal_id,
                &request.get_ref().actions,
            )
            .await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::permissions::get", skip(self))]
    async fn get(
        &self,
        request: Request<GetPermissionInfoRequest>,
    ) -> Result<Response<PermissionInfo>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let resource_id = Self::parse_uuid(&request.get_ref().resource_id)?;
        let principal_id = Self::parse_uuid(&request.get_ref().principal_id)?;
        let res = self.mgr.get(&claims, &resource_id, &principal_id).await?;
        Ok(Response::new(res))
    }

    type ListStream =
        Pin<Box<dyn Stream<Item = Result<PermissionInfo, Status>> + Send + Sync + 'static>>;

    #[tracing::instrument(name = "svc::permissions::list", skip(self))]
    async fn list(
        &self,
        request: Request<ListPermissionsRequest>,
    ) -> Result<Response<Self::ListStream>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let resource_id = Self::parse_uuid(&request.get_ref().resource_id)?;
        let res = self.mgr.list(&claims, &resource_id).await?;
        Ok(Response::new(res))
    }
}
