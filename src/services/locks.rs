use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api;
use api::catalog::locks_server::Locks;
use api::catalog::{LockRequest, LockResponse};

use crate::managers;
use crate::token;

use super::base::BaseService;

#[derive(Debug)]
pub struct Service {
    mgr: Arc<managers::locks::Manager>,
    validator: Arc<token::Validator>,
}

impl BaseService for Service {}

impl Service {
    pub fn new(
        mgr: Arc<managers::locks::Manager>,
        validator: Arc<token::Validator>,
    ) -> Result<Service, sqlx::Error> {
        let res = Service {
            mgr: mgr,
            validator: validator,
        };
        Ok(res)
    }
}

#[tonic::async_trait]
impl Locks for Service {
    type LockStream =
        Pin<Box<dyn Stream<Item = Result<LockResponse, Status>> + Send + Sync + 'static>>;

    #[tracing::instrument(name = "svc::locks::lock", skip(self))]
    async fn lock(
        &self,
        request: Request<LockRequest>,
    ) -> Result<Response<Self::LockStream>, Status> {
        self.validator.get_access_token_claims(&request)?;
        let res = self.mgr.lock(&request.get_ref().lock_id).await?;
        Ok(Response::new(res))
    }

    type TryLockStream =
        Pin<Box<dyn Stream<Item = Result<LockResponse, Status>> + Send + Sync + 'static>>;

    #[tracing::instrument(name = "svc::locks::try_lock", skip(self))]
    async fn try_lock(
        &self,
        request: Request<LockRequest>,
    ) -> Result<Response<Self::TryLockStream>, Status> {
        self.validator.get_access_token_claims(&request)?;
        let res = self.mgr.try_lock(&request.get_ref().lock_id).await?;
        Ok(Response::new(res))
    }
}
