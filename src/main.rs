use clap::Clap;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tonic::transport::Server;
use tracing_subscriber::util::SubscriberInitExt;

#[macro_use]
extern crate lazy_static;

pub mod api {
    pub mod catalog {
        tonic::include_proto!("catalog");
        pub(crate) const CATALOG_FILE_DESCRIPTOR_SET: &'static [u8] =
            tonic::include_file_descriptor_set!("catalog_descriptor");
    }

    pub mod idp {
        tonic::include_proto!("idp");
        pub(crate) const IDP_FILE_DESCRIPTOR_SET: &'static [u8] =
            tonic::include_file_descriptor_set!("idp_descriptor");
    }
}

mod managers;
mod services;
mod token;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    info!("starting up...");

    let pool = Arc::new(
        PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(30))
            .connect(&OPTS.database)
            .await?,
    );

    let validator = Arc::new(token::Validator::new(&OPTS.secret));

    let schemas_mgr = Arc::new(managers::schemas::Manager::new(pool.clone()).await?);
    let permissions_mgr = Arc::new(managers::permissions::Manager::new(pool.clone()).await?);
    let locks_mgr = Arc::new(managers::locks::Manager::new(pool.clone()).await?);
    let events_mgr = Arc::new(
        managers::events::Manager::new(pool.clone(), permissions_mgr.clone(), &OPTS.database)
            .await?,
    );
    let resources_mgr = Arc::new(
        managers::resources::Manager::new(
            pool.clone(),
            permissions_mgr.clone(),
            schemas_mgr.clone(),
            events_mgr.clone(),
        )
        .await?,
    );
    let users_mgr = Arc::new(managers::users::Manager::new(pool.clone()).await?);

    let service_accounts_mgr =
        Arc::new(managers::service_accounts::Manager::new(pool.clone()).await?);

    let groups_mgr =
        Arc::new(managers::groups::Manager::new(pool.clone(), users_mgr.clone()).await?);

    let schemas_svc = services::schemas::Service::new(schemas_mgr.clone(), validator.clone())?;

    let resources_svc =
        services::resources::Service::new(resources_mgr.clone(), validator.clone())?;

    let permissions_svc =
        services::permissions::Service::new(permissions_mgr.clone(), validator.clone())?;

    let events_svc = services::events::Service::new(validator.clone(), events_mgr.clone())?;

    let locks_svc = services::locks::Service::new(locks_mgr.clone(), validator.clone())?;

    let users_svc = services::users::Service::new(users_mgr.clone(), validator.clone())?;

    let service_accounts_svc =
        services::service_accounts::Service::new(service_accounts_mgr.clone(), validator.clone())?;

    let groups_svc = services::groups::Service::new(groups_mgr.clone(), validator.clone())?;

    let auth_svc = services::auth::Service::new(
        users_mgr.clone(),
        groups_mgr.clone(),
        service_accounts_mgr.clone(),
        validator.clone(),
        &OPTS.secret.clone(),
    )?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(api::catalog::CATALOG_FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(api::idp::IDP_FILE_DESCRIPTOR_SET)
        .build()?;

    create_admin_service_account(service_accounts_mgr.clone()).await?;

    let config = tonic_web::config().allow_origins(vec![
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
            config.enable(api::catalog::resources_server::ResourcesServer::new(
                resources_svc,
            )),
        )
        .add_service(
            config.enable(api::catalog::schemas_server::SchemasServer::new(
                schemas_svc,
            )),
        )
        .add_service(config.enable(api::catalog::events_server::EventsServer::new(events_svc)))
        .add_service(config.enable(api::catalog::locks_server::LocksServer::new(locks_svc)))
        .add_service(
            config.enable(api::catalog::permissions_server::PermissionsServer::new(
                permissions_svc,
            )),
        )
        .add_service(config.enable(api::idp::users_server::UsersServer::new(users_svc)))
        .add_service(config.enable(api::idp::groups_server::GroupsServer::new(groups_svc)))
        .add_service(config.enable(
            api::idp::service_accounts_server::ServiceAccountsServer::new(service_accounts_svc),
        ))
        .add_service(
            config.enable(api::idp::authentication_server::AuthenticationServer::new(
                auth_svc,
            )),
        )
        .add_service(reflection_service)
        .serve(OPTS.listen.parse()?)
        .await?;

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
