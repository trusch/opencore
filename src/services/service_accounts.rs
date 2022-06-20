use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api;

use api::idp::service_accounts_server::ServiceAccounts;
use api::idp::{
    CreateServiceAccountRequest, CreateServiceAccountResponse, DeleteServiceAccountRequest,
    GetServiceAccountRequest, ListServiceAccountsRequest, ServiceAccount,
    UpdateServiceAccountRequest, UpdateServiceAccountResponse,
};

use crate::managers::service_accounts;
use crate::token;

pub struct Service {
    mgr: Arc<service_accounts::Manager>,
    validator: Arc<token::Validator>,
}

impl Service {
    pub fn new(
        mgr: Arc<service_accounts::Manager>,
        validator: Arc<token::Validator>,
    ) -> Result<Service, sqlx::Error> {
        let res = Service { mgr, validator };
        Ok(res)
    }
}

#[tonic::async_trait]
impl ServiceAccounts for Service {
    #[tracing::instrument(name = "svc::service_accounts::create", skip(self))]
    async fn create(
        &self,
        request: Request<CreateServiceAccountRequest>,
    ) -> Result<Response<CreateServiceAccountResponse>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins can manage service accounts",
            ));
        }

        let r = request.get_ref();

        let res = self.mgr.create(&claims, &r.name, r.is_admin).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::service_accounts::get", skip(self))]
    async fn get(
        &self,
        request: Request<GetServiceAccountRequest>,
    ) -> Result<Response<ServiceAccount>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins can manage service accounts",
            ));
        }

        use uuid::Uuid;
        let service_account_id = match Uuid::parse_str(&request.get_ref().id) {
            Ok(id) => id,
            Err(_) => Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, request.get_ref().id.as_bytes()),
        };
        let id = sqlx::types::Uuid::from_bytes(service_account_id.into_bytes());
        
        let res = self.mgr.get(&claims, &id).await?;

        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::service_accounts::delete", skip(self))]
    async fn delete(
        &self,
        request: Request<DeleteServiceAccountRequest>,
    ) -> Result<Response<ServiceAccount>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins can manage service accounts",
            ));
        }

        use uuid::Uuid;
        let id = match Uuid::parse_str(&request.get_ref().id) {
            Ok(id) => id,
            Err(_) => Uuid::new_v5(&Uuid::NAMESPACE_OID, request.get_ref().id.as_bytes()),
        };
        let id = sqlx::types::Uuid::from_bytes(id.into_bytes());


        let res = self.mgr.delete(&claims, &id).await?;

        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::service_accounts::update", skip(self))]
    async fn update(
        &self,
        request: Request<UpdateServiceAccountRequest>,
    ) -> Result<Response<UpdateServiceAccountResponse>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins can manage service accounts",
            ));
        }

        let r = request.get_ref();

        use uuid::Uuid;
        let id = match Uuid::parse_str(&r.id) {
            Ok(id) => id,
            Err(_) => Uuid::new_v5(&Uuid::NAMESPACE_OID, request.get_ref().id.as_bytes()),
        };
        let id = sqlx::types::Uuid::from_bytes(id.into_bytes());

        let result = self.mgr.update(&claims, &id).await?;
        Ok(Response::new(result))
    }

    type ListStream =
        Pin<Box<dyn Stream<Item = Result<ServiceAccount, Status>> + Send + Sync + 'static>>;

    #[tracing::instrument(name = "svc::service_accounts::list", skip(self))]
    async fn list(
        &self,
        request: Request<ListServiceAccountsRequest>,
    ) -> Result<Response<Self::ListStream>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins can manage service accounts",
            ));
        }

        let res = self.mgr.list().await?;

        Ok(Response::new(res))
    }
}
