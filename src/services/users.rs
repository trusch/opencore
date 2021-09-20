use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api;

use api::idp::users_server::Users;
use api::idp::{
    CreateUserRequest, DeleteUserRequest, GetUserRequest, ListUsersRequest, UpdateUserRequest, User,
};

use crate::token;

use crate::managers;

use super::base::BaseService;

#[derive(Debug)]
pub struct Service {
    mgr: Arc<managers::users::Manager>,
    validator: Arc<token::Validator>,
}

impl BaseService for Service {}

impl Service {
    pub fn new(
        mgr: Arc<managers::users::Manager>,
        validator: Arc<token::Validator>,
    ) -> Result<Service, sqlx::Error> {
        let res = Service {
            mgr,
            validator,
        };
        Ok(res)
    }
}

#[tonic::async_trait]
impl Users for Service {
    #[tracing::instrument(name = "svc::users::create", skip(self))]
    async fn create(&self, request: Request<CreateUserRequest>) -> Result<Response<User>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        if !claims.adm {
            return Err(Status::permission_denied(
                "only admins can create new users",
            ));
        }

        let r = request.get_ref();

        if r.name.is_empty() {
            return Err(Status::invalid_argument("'name' must be specified"));
        }
        if r.email.is_empty() {
            return Err(Status::invalid_argument("'email' must be specified"));
        }
        if r.password.len() < 8 {
            return Err(Status::invalid_argument(
                "'password' must be at least 8 byte long",
            ));
        }

        let res = self
            .mgr
            .create(&claims, &r.name, &r.email, &r.is_admin, &r.password)
            .await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::users::get", skip(self))]
    async fn get(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
        let r = request.get_ref();
        let selector = match () {
            _ if !r.id.is_empty() => {
                let user_id = Self::parse_uuid(&request.get_ref().id)?;
                managers::users::GetSelector::ById(user_id)
            }
            _ if !r.email.is_empty() => managers::users::GetSelector::ByEmail(r.email.clone()),
            _ => {
                return Err(Status::invalid_argument(
                    "you need to supply 'id' or 'email'",
                ));
            }
        };

        let res = self.mgr.get(selector).await?;

        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::users::delete", skip(self))]
    async fn delete(&self, request: Request<DeleteUserRequest>) -> Result<Response<User>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        if !claims.adm {
            return Err(Status::permission_denied("only admins can delete users"));
        }

        let id = Self::parse_uuid(&request.get_ref().id)?;

        let res = self.mgr.delete(&claims, &id).await?;

        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::users::update", skip(self))]
    async fn update(&self, request: Request<UpdateUserRequest>) -> Result<Response<User>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        let r = request.get_ref();

        if !claims.adm && r.id != claims.sub {
            return Err(Status::permission_denied(
                "only admins can update users other than their own",
            ));
        }

        if !r.password.is_empty() && r.password.len() < 8 {
            return Err(Status::invalid_argument(
                "'password' must be at least 8 byte long",
            ));
        }

        let id = Self::parse_uuid(&request.get_ref().id)?;

        let result = self
            .mgr
            .update(&claims, &id, &r.name, &r.email, &r.password)
            .await?;
        Ok(Response::new(result))
    }

    type ListStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send + Sync + 'static>>;

    #[tracing::instrument(name = "svc::users::list", skip(self))]
    async fn list(
        &self,
        _request: Request<ListUsersRequest>,
    ) -> Result<Response<Self::ListStream>, Status> {
        let res = self.mgr.list().await?;
        Ok(Response::new(res))
    }
}
