use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api;
use crate::token;

use api::idp::groups_server::Groups;
use api::idp::{
    AddUserToGroupRequest, CreateGroupRequest, DelUserFromGroupRequest, DeleteGroupRequest,
    GetGroupRequest, Group, GroupMember, ListGroupMembersRequest, ListGroupsRequest,
    UpdateGroupRequest,
};

use super::base::BaseService;
use crate::managers::groups;

pub struct Service {
    mgr: Arc<groups::Manager>,
    validator: Arc<token::Validator>,
}

impl BaseService for Service {}

impl Service {
    pub fn new(
        mgr: Arc<groups::Manager>,
        validator: Arc<token::Validator>,
    ) -> Result<Service, sqlx::Error> {
        let res = Service { mgr, validator };
        Ok(res)
    }
}

#[tonic::async_trait]
impl Groups for Service {
    #[tracing::instrument(name = "svc::groups::create", skip(self))]
    async fn create(
        &self,
        request: Request<CreateGroupRequest>,
    ) -> Result<Response<Group>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        let r = request.get_ref();

        let res = self.mgr.create(&claims, &r.name).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::groups::get", skip(self))]
    async fn get(&self, request: Request<GetGroupRequest>) -> Result<Response<Group>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        let id = Self::parse_uuid(&request.get_ref().id)?;

        let res = self.mgr.get(&claims, &id).await?;

        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::groups::delete", skip(self))]
    async fn delete(
        &self,
        request: Request<DeleteGroupRequest>,
    ) -> Result<Response<Group>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        let id = Self::parse_uuid(&request.get_ref().id)?;

        let res = self.mgr.delete(&claims, &id).await?;

        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::groups::update", skip(self))]
    async fn update(
        &self,
        request: Request<UpdateGroupRequest>,
    ) -> Result<Response<Group>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        let r = request.get_ref();
        let id = Self::parse_uuid(&request.get_ref().id)?;
        let result = self.mgr.update(&claims, &id, &r.name).await?;

        Ok(Response::new(result))
    }

    type ListStream = Pin<Box<dyn Stream<Item = Result<Group, Status>> + Send + Sync + 'static>>;

    #[tracing::instrument(name = "svc::groups::list", skip(self))]
    async fn list(
        &self,
        request: Request<ListGroupsRequest>,
    ) -> Result<Response<Self::ListStream>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        let res = self.mgr.list(&claims).await?;

        Ok(Response::new(res))
    }

    type ListMembersStream =
        Pin<Box<dyn Stream<Item = Result<GroupMember, Status>> + Send + Sync + 'static>>;

    #[tracing::instrument(name = "svc::groups::list_members", skip(self))]
    async fn list_members(
        &self,
        request: Request<ListGroupMembersRequest>,
    ) -> Result<Response<Self::ListMembersStream>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        let id = Self::parse_uuid(&request.get_ref().group_id)?;
        let res = self.mgr.list_members(&claims, &id).await?;

        Ok(Response::new(res))
    }

    #[tracing::instrument(name = "svc::groups::add_user", skip(self))]
    async fn add_user(
        &self,
        request: Request<AddUserToGroupRequest>,
    ) -> Result<Response<()>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;

        let r = request.get_ref();

        let user_id = Self::parse_uuid(&request.get_ref().user_id)?;

        let group_id = Self::parse_uuid(&request.get_ref().group_id)?;

        self.mgr
            .add_user(&claims, &user_id, &group_id, &r.is_admin)
            .await?;

        Ok(Response::new(()))
    }

    #[tracing::instrument(name = "svc::groups::del_user", skip(self))]
    async fn del_user(
        &self,
        request: Request<DelUserFromGroupRequest>,
    ) -> Result<Response<()>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let r = request.get_ref();
        let user_id = Self::parse_uuid(&r.user_id)?;
        let group_id = Self::parse_uuid(&r.group_id)?;

        self.mgr.remove_user(&claims, &user_id, &group_id).await?;

        Ok(Response::new(()))
    }
}
