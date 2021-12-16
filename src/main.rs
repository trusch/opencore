use clap::Clap;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tonic::transport::Server;
use tracing_subscriber::util::SubscriberInitExt;

#[macro_use]
extern crate lazy_static;

use opencore::api;
use opencore::managers;
use opencore::services;
use opencore::token;

#[derive(Clap)]
#[clap(version = "0.1", author = "Tino Rusch <tino.rusch@gmail.com>")]
struct Opts {
    #[clap(short, long, default_value = "postgres://postgres:password@localhost")]
    database: String,
    #[clap(short, long, default_value = "127.0.0.1:3001")]
    listen: String,
    #[clap(short, long, default_value = "secret")]
    secret: String,
}

lazy_static! {
    static ref OPTS: Opts = Opts::parse();
}

struct ManagersContainer {
    pub validator: Arc<token::Validator>,
    pub schemas: Arc<managers::schemas::Manager>,
    pub permissions: Arc<managers::permissions::Manager>,
    pub locks: Arc<managers::locks::Manager>,
    pub events: Arc<managers::events::Manager>,
    pub resources: Arc<managers::resources::Manager>,
    pub users: Arc<managers::users::Manager>,
    pub service_accounts: Arc<managers::service_accounts::Manager>,
    pub groups: Arc<managers::groups::Manager>,
}

impl ManagersContainer {
    pub async fn new() -> Result<ManagersContainer, Box<dyn std::error::Error>> {
        let pool = Arc::new(
            PgPoolOptions::new()
                .connect_timeout(std::time::Duration::from_secs(30))
                .connect(&OPTS.database)
                .await?,
        );

        let validator = Arc::new(token::Validator::new(&OPTS.secret));
        let schemas = Arc::new(managers::schemas::Manager::new(pool.clone()).await?);
        let permissions = Arc::new(managers::permissions::Manager::new(pool.clone()).await?);
        let locks = Arc::new(managers::locks::Manager::new(pool.clone()).await?);
        let events = Arc::new(
            managers::events::Manager::new(pool.clone(), permissions.clone(), &OPTS.database)
                .await?,
        );
        let resources = Arc::new(
            managers::resources::Manager::new(
                pool.clone(),
                permissions.clone(),
                schemas.clone(),
                events.clone(),
                locks.clone(),
            )
            .await?,
        );
        let users = Arc::new(managers::users::Manager::new(pool.clone()).await?);

        let service_accounts =
            Arc::new(managers::service_accounts::Manager::new(pool.clone()).await?);

        let groups = Arc::new(managers::groups::Manager::new(pool.clone(), users.clone()).await?);

        Ok(Self {
            validator,
            schemas,
            permissions,
            locks,
            events,
            resources,
            users,
            service_accounts,
            groups,
        })
    }
}

struct ServicesContainer {
    pub schemas: services::schemas::Service,
    pub resources: services::resources::Service,
    pub permissions: services::permissions::Service,
    pub events: services::events::Service,
    pub locks: services::locks::Service,
    pub users: services::users::Service,
    pub service_accounts: services::service_accounts::Service,
    pub groups: services::groups::Service,
    pub auth: services::auth::Service,
}

impl ServicesContainer {
    pub async fn new(
        managers: &ManagersContainer,
    ) -> Result<ServicesContainer, Box<dyn std::error::Error>> {
        let schemas =
            services::schemas::Service::new(managers.schemas.clone(), managers.validator.clone())?;

        let resources = services::resources::Service::new(
            managers.resources.clone(),
            managers.validator.clone(),
        )?;

        let permissions = services::permissions::Service::new(
            managers.permissions.clone(),
            managers.validator.clone(),
        )?;

        let events =
            services::events::Service::new(managers.events.clone(), managers.validator.clone())?;

        let locks =
            services::locks::Service::new(managers.locks.clone(), managers.validator.clone())?;

        let users =
            services::users::Service::new(managers.users.clone(), managers.validator.clone())?;

        let service_accounts = services::service_accounts::Service::new(
            managers.service_accounts.clone(),
            managers.validator.clone(),
        )?;

        let groups =
            services::groups::Service::new(managers.groups.clone(), managers.validator.clone())?;

        let auth = services::auth::Service::new(
            managers.users.clone(),
            managers.groups.clone(),
            managers.service_accounts.clone(),
            managers.validator.clone(),
            &OPTS.secret.clone(),
        )?;

        Ok(Self {
            schemas,
            resources,
            permissions,
            events,
            locks,
            users,
            service_accounts,
            groups,
            auth,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing()?;

    info!("starting up...");

    let managers = ManagersContainer::new().await?;

    let services = ServicesContainer::new(&managers).await?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(api::catalog::CATALOG_FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(api::idp::IDP_FILE_DESCRIPTOR_SET)
        .build()?;

    create_admin_service_account(managers.service_accounts.clone()).await?;

    let web_config = tonic_web::config().allow_origins(vec![
        OPTS.listen.clone(),
        "http://127.0.0.1:8080".to_string(),
        "http://127.0.0.1:3000".to_string(),
        "http://127.0.0.1".to_string(),
        "http://localhost:8080".to_string(),
        "http://localhost:3000".to_string(),
        "http://localhost".to_string(),
    ]);

    Server::builder()
        .accept_http1(true)
        .add_service(
            web_config.enable(api::catalog::resources_server::ResourcesServer::new(
                services.resources,
            )),
        )
        .add_service(
            web_config.enable(api::catalog::schemas_server::SchemasServer::new(
                services.schemas,
            )),
        )
        .add_service(
            web_config.enable(api::catalog::events_server::EventsServer::new(
                services.events,
            )),
        )
        .add_service(
            web_config.enable(api::catalog::locks_server::LocksServer::new(services.locks)),
        )
        .add_service(
            web_config.enable(api::catalog::permissions_server::PermissionsServer::new(
                services.permissions,
            )),
        )
        .add_service(web_config.enable(api::idp::users_server::UsersServer::new(services.users)))
        .add_service(web_config.enable(api::idp::groups_server::GroupsServer::new(services.groups)))
        .add_service(web_config.enable(
            api::idp::service_accounts_server::ServiceAccountsServer::new(
                services.service_accounts,
            ),
        ))
        .add_service(
            web_config.enable(api::idp::authentication_server::AuthenticationServer::new(
                services.auth,
            )),
        )
        .add_service(reflection_service)
        .serve(OPTS.listen.parse()?)
        .await?;

    Ok(())
}

fn setup_tracing() -> Result<(), Box<dyn std::error::Error>> {
    use opentelemetry::global;
    use opentelemetry::sdk::trace::{self, IdGenerator, Sampler};

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_collector_endpoint("http://localhost:14268/api/traces")
        .with_service_name("opencore")
        .with_max_packet_size(65000)
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(IdGenerator::default()),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::EnvFilter;

    let fmt_layer = tracing_subscriber::fmt::Layer::new()
        .event_format(tracing_subscriber::fmt::format().compact());

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(telemetry)
        .with(fmt_layer)
        .try_init()?;

    Ok(())
}

async fn create_admin_service_account(
    sa_mgr: Arc<managers::service_accounts::Manager>,
) -> Result<(), tonic::Status> {
    match sa_mgr.create(&token::Claims::admin(), "root", true).await {
        Ok(root) => {
            println!("Created admin service account.");
            println!("OPENCORE_ADMIN_NAME=root");
            println!("OPENCORE_ADMIN_KEY='{}'", root.secret_key);
        }
        Err(err) => {
            println!("root service account already exists: {}.", err);
        }
    };
    Ok(())
}
