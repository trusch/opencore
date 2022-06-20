use jsonwebtoken::{encode, EncodingKey, Header};
use kilt_api_client::{RuntimeApi, KiltConfig};
use kilt_api_client::runtime_types::did::did_details::{DidPublicKey, DidVerificationKey};
use pwhash::bcrypt;
use sp_core::{Pair};
use sqlx::types::Uuid;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use sp_core::crypto::{Ss58Codec, AccountId32};

use crate::api;

use api::idp::authentication_server::Authentication;
use api::idp::{LoginRequest, LoginResponse, RefreshRequest};

use crate::managers::groups;
use crate::managers::service_accounts;
use crate::managers::users;
use crate::token;

use token::Claims;

use super::base::BaseService;

pub type KiltRuntimeApi = RuntimeApi<KiltConfig, subxt::PolkadotExtrinsicParams<KiltConfig>>;

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
        if !request.get_ref().service_account_id.is_empty() {
            tracing::info!("authenticate service account");
            return self.login_by_service_account(request).await
        } else if request.get_ref().did_login.is_some() {
            tracing::info!("authenticate did");
            self.login_by_did(request).await
        }else {
            tracing::info!("authenticate user");
            self.login_by_password(request).await
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

        match self.user_manager.get(&Claims::admin(), users::GetSelector::ById(id)).await {
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

    async fn login_by_password(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        // get user
        let user = self
            .user_manager
            .get(&Claims::admin(), users::GetSelector::ByExternalId(request.get_ref().external_id.clone()))
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
    }

    async fn login_by_service_account(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        // convert id to UUID
        let raw_id = &request.get_ref().service_account_id;

        use uuid::Uuid;
        let id = match Uuid::parse_str(raw_id) {
            Ok(id) => id,
            Err(_) => Uuid::new_v5(&Uuid::NAMESPACE_OID, raw_id.as_bytes()),
        };
        let id = sqlx::types::Uuid::from_bytes(id.into_bytes());
        
        // get service account
        let sa = self
            .service_account_manager
            .get(&Claims::admin(), &id)
            .await?;

        // check password
        let ok = bcrypt::verify(&request.get_ref().password, &sa.secret_key_hash);
        if !ok {
            return Err(Status::unauthenticated("wrong password"));
        }

        // generate and return token
        Ok(Response::new(LoginResponse {
            access_token: self.create_token(&sa.id, sa.is_admin, &[], false)?,
            refresh_token: self.create_token(&sa.id, sa.is_admin, &[], true)?,
        }))
    }

    async fn login_by_did(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let did_login = match &request.get_ref().did_login {
            Some(msg) => msg,
            None => return Err(Status::unauthenticated("no did login message")) 
        };
        
        let sig_string = did_login.signature.trim_start_matches("0x");

        let sig = match hex::decode(sig_string){
            Ok(sig) => match sig.try_into() {
                Ok(sig) => sp_core::sr25519::Signature(sig),
                Err(_) => {
                    tracing::error!("failed to convert signature");
                    return Err(Status::unauthenticated("failed to convert signature"));
                }
            },
            Err(_) => return Err(Status::unauthenticated("invalid signature (bad hex)")),
        };

        let api = match kilt_api_client::connect("wss://spiritnet.kilt.io:443").await {
            Ok(api) => api,
            Err(err) => return Err(Status::failed_precondition(format!("failed to connect to kilt: {}", err)))
        };

        let pub_key = self.get_pubkey_for_did(&api, &request.get_ref().external_id).await?;

        let ok = sp_core::sr25519::Pair::verify(&sig, did_login.message.as_bytes(), &pub_key);
        if !ok {
            return Err(Status::unauthenticated("can not verify signature"));
        }

        let user = match self
            .user_manager
            .get(&Claims::admin(), users::GetSelector::ByExternalId(request.get_ref().external_id.clone()))
            .await {
            Ok(user) => user,
            Err(_) => {
                let name = self.get_w3n(&api, &request.get_ref().external_id).await?;
                self.user_manager.create(
                    &Claims::admin(), 
                    &name, 
                    &request.get_ref().external_id, 
                    &false, 
                    &"",
                ).await?
            },
        };

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
    

    }

    #[tracing::instrument(name = "svc::auth::get_key_for_did", skip(self, api))]
    async fn get_pubkey_for_did(&self, api: &KiltRuntimeApi, did: &str) -> Result<sp_core::sr25519::Public, Status> {
        let did_account = did.into_account_id()?;
        
        let details = match api.storage().did().did(&did_account, None).await {
            Ok(Some(details)) => details,
            Ok(None) | Err(_) => return Err(Status::failed_precondition(format!("failed to get did details")))
        };

        let k = details.public_keys.0.iter()
            .filter(|(k,_)|*k==details.authentication_key)
            .map(|(_, v)| &v.key)
            .take(1)
            .collect::<Vec<&DidPublicKey>>()[0];

        let pub_key = if let DidPublicKey::PublicVerificationKey(DidVerificationKey::Sr25519(pub_key)) = k {
            pub_key
        }else{
            return Err(Status::failed_precondition(format!("failed to get did details")))
        };

        Ok(sp_core::sr25519::Public(pub_key.0))
    }

    #[tracing::instrument(name = "svc::auth::get_w3n", skip(self, api))]
    async fn get_w3n(&self, api: &KiltRuntimeApi, did: &str) -> Result<String, Status> {
        match api.storage().web3_names().names(&did.into_account_id()?, None).await {
            Ok(Some(name)) => match String::from_utf8(name.0.0) {
                Ok(name) => Ok(name),
                Err(err) => Err(Status::failed_precondition(format!("failed to parse web3name: {}", err)))
            },
            Ok(None) | Err(_) => Err(Status::failed_precondition(format!("failed to get web3name")))
        }
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

trait IntoAccountId {
    fn into_account_id(self) -> Result<AccountId32, Status>;
}


const DID_PREFIX: &str = "did:kilt:";

impl<T> IntoAccountId for T where T: Into<String> {
    fn into_account_id(self) -> Result<AccountId32, Status> {
        let s: String = self.into();
        if !s.starts_with(DID_PREFIX) {
            return Err(Status::failed_precondition("can't parse did string"));
        }
        let ss58_address: String = s[DID_PREFIX.len()..].into();
        AccountId32::from_ss58check(&ss58_address)
            .map_err(|e| 
                Status::failed_precondition(format!("failed to parse did: {}", e)
            )
        )
    }
}