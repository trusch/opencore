use jsonwebtoken::{encode, EncodingKey, Header};
use pwhash::bcrypt;
use sqlx::types::Uuid;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::api;

use api::idp::authentication_server::Authentication;
use api::idp::{LoginRequest, LoginResponse, RefreshRequest};

use crate::managers::groups;
use crate::managers::service_accounts;
use crate::managers::users;
use crate::token;

use token::Claims;

use super::base::BaseService;

const TOKEN_LIFETIME: usize = 120;
const REFRESH_LIFETIME: usize = 60 * 60 * 24;

pub struct Service {
    user_manager: Arc<users::Manager>,
    service_account_manager: Arc<service_accounts::Manager>,
    group_manager: Arc<groups::Manager>,
    validator: Arc<token::Validator>,
    key: String,
}

impl BaseService for Service {}

#[tonic::async_trait]
impl Authentication for Service {
    #[tracing::instrument(name = "svc::auth::login", skip(self))]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        if !request.get_ref().email.is_empty() {
            // get user
            let user = self
                .user_manager
                .get(users::GetSelector::ByEmail(request.get_ref().email.clone()))
                .await?;

            // check password
            let ok = bcrypt::verify(&request.get_ref().password, &user.password_hash);
            if !ok {
                return Err(Status::unauthenticated("wrong password"));
            }

            // convert id to UUID
            let id = match Uuid::parse_str(&user.id) {
                Ok(id) => id,
                Err(err) => {
                    return Err(Status::internal(format!("failed to parse id: {}", err)));
                }
            };

            // list groups and extract list of id's
            let grps = self
                .group_manager
                .list_as_vector(&id)
                .await?
                .iter()
                .map(|grp| grp.id.clone())
                .collect::<Vec<String>>();

            // generate and return token
            Ok(Response::new(LoginResponse {
                access_token: self.create_token(&user.id, user.is_admin, &grps, false)?,
                refresh_token: self.create_token(&user.id, user.is_admin, &grps, true)?,
            }))
        } else {
            // convert id to UUID
            let raw_id = &request.get_ref().service_account_id;
            let id = match Uuid::parse_str(raw_id) {
                Ok(id) => id,
                Err(_) => Uuid::new_v5(&Uuid::NAMESPACE_OID, raw_id.as_bytes()),
            };

            // get service account
            let sa = self
                .service_account_manager
                .get(&Claims::admin(), &id)
                .await?;
            // generate and return token
            Ok(Response::new(LoginResponse {
                access_token: self.create_token(&sa.id, sa.is_admin, &[], false)?,
                refresh_token: self.create_token(&sa.id, sa.is_admin, &[], true)?,
            }))
        }
    }

    #[tracing::instrument(name = "svc::auth::refresh", skip(self))]
    async fn refresh(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let claims = self.validator.validate(&request.get_ref().refresh_token)?;
        if !claims.rfs {
            // token is NOT a refesh token
            use crate::token::error::Error::Validate;
            return Err(Validate("expected refresh token".to_string()).into());
        }

        // convert id to UUID
        let id = match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(err) => {
                return Err(Status::internal(format!("failed to parse id: {}", err)));
            }
        };

        match self
            .user_manager
            .get(users::GetSelector::ById(id))
            .await
        {
            Ok(user) => {
                let grps = self
                    .group_manager
                    .list_as_vector(&id)
                    .await?
                    .iter()
                    .map(|grp| grp.id.clone())
                    .collect::<Vec<String>>();
                Ok(Response::new(LoginResponse {
                    access_token: self.create_token(&user.id, user.is_admin, &grps, false)?,
                    refresh_token: self.create_token(&user.id, user.is_admin, &grps, true)?,
                }))
            }
            Err(_) => {
                let sa = self
                    .service_account_manager
                    .get(&Claims::admin(), &id)
                    .await?;
                Ok(Response::new(LoginResponse {
                    access_token: self.create_token(&sa.id, sa.is_admin, &[], false)?,
                    refresh_token: self.create_token(&sa.id, sa.is_admin, &[], true)?,
                }))
            }
        }
    }
}

impl Service {
    pub fn new(
        user_manager: Arc<users::Manager>,
        group_manager: Arc<groups::Manager>,
        service_account_manager: Arc<service_accounts::Manager>,
        validator: Arc<token::Validator>,
        key: &str,
    ) -> Result<Service, sqlx::Error> {
        let res = Service {
            user_manager,
            group_manager,
            service_account_manager,
            validator,
            key: key.to_string(),
        };
        Ok(res)
    }

    #[tracing::instrument(name = "svc::auth::create_token", skip(self))]
    fn create_token(
        &self,
        sub: &str,
        is_admin: bool,
        groups: &[String],
        is_refresh: bool,
    ) -> Result<String, Status> {
        let mut exp = TOKEN_LIFETIME;
        if is_refresh {
            exp = REFRESH_LIFETIME;
        }
        let now = chrono::Utc::now();
        let claims = token::Claims {
            iss: "opencore".to_string(),
            exp: now.timestamp() as usize + exp,
            iat: now.timestamp() as usize,
            nbf: now.timestamp() as usize,
            sub: sub.to_string(),
            grp: groups.to_vec(),
            adm: is_admin,
            rfs: is_refresh,
        };

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.key.as_ref()),
        ) {
            Ok(token) => token,
            Err(err) => {
                return Err(Status::internal(format!("failed to encode token: {}", err)));
            }
        };

        Ok(token)
    }
}
