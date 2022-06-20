use clap::Parser;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tonic::transport::Server;
use tracing_subscriber::util::SubscriberInitExt;

use axum::{body::BoxBody, http::StatusCode, routing::get_service, Router};
use futures::ready;
use hyper::service::make_service_fn;
use hyper::{Body, Request, Response};
use pin_project::pin_project;
use std::future::Future;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::task::Poll;
use tower::{Service, ServiceExt};
use tower_http::cors::{Any, CorsLayer};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[macro_use]
extern crate lazy_static;

use opencore::api;
use opencore::managers;
use opencore::services;
use opencore::token;

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Tino Rusch <tino.rusch@gmail.com>")]
struct Opts {
    #[clap(short, long, default_value = "postgres://postgres:password@localhost")]
    database: String,
    #[clap(short, long, default_value = "127.0.0.1:3001")]
    listen: String,
    #[clap(short, long, default_value = "secret")]
    secret: String,
    #[clap(long, default_value = "127.0.0.1:3000")]
    allow_origins: Vec<String>,
    #[clap(long, default_value = ".")]
    static_dir: String,
    #[clap(long, default_value = "")]
    schema_dir: String,
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

        let groups = Arc::new(managers::groups::Manager::new(pool.clone()).await?);

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

    if OPTS.schema_dir.len() > 0 {
        info!("loading schemas from directory {}...", &OPTS.schema_dir);
        managers.schemas.load_from_directory(&OPTS.schema_dir).await?;
    }

    let services = ServicesContainer::new(&managers).await?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(api::catalog::CATALOG_FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(api::idp::IDP_FILE_DESCRIPTOR_SET)
        .build()?;

    create_admin_service_account(managers.service_accounts.clone()).await?;

    let grpc_web_config = tonic_web::config().allow_all_origins();

    let grpc_service = Server::builder()
        .accept_http1(true)
        .add_service(
            grpc_web_config.enable(api::catalog::resources_server::ResourcesServer::new(
                services.resources,
            )),
        )
        .add_service(
            grpc_web_config.enable(api::catalog::schemas_server::SchemasServer::new(
                services.schemas,
            )),
        )
        .add_service(
            grpc_web_config.enable(api::catalog::events_server::EventsServer::new(
                services.events,
            )),
        )
        .add_service(
            grpc_web_config.enable(api::catalog::locks_server::LocksServer::new(services.locks)),
        )
        .add_service(grpc_web_config.enable(
            api::catalog::permissions_server::PermissionsServer::new(services.permissions),
        ))
        .add_service(
            grpc_web_config.enable(api::idp::users_server::UsersServer::new(services.users)),
        )
        .add_service(
            grpc_web_config.enable(api::idp::groups_server::GroupsServer::new(services.groups)),
        )
        .add_service(grpc_web_config.enable(
            api::idp::service_accounts_server::ServiceAccountsServer::new(
                services.service_accounts,
            ),
        ))
        .add_service(grpc_web_config.enable(
            api::idp::authentication_server::AuthenticationServer::new(services.auth),
        ))
        .add_service(reflection_service)
        .into_service()
        // Needed to unify body type into axum's BoxBody
        .map_response(|response| {
            let (parts, body) = response.into_parts();
            Response::from_parts(parts, axum::body::boxed(body))
        });

    let axum_service = Router::new()
        .nest(
            "/",
            get_service(ServeDir::new(&OPTS.static_dir)).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers(Any))
        // Needed to unify errors types from Infallible to tonic's Box<dyn std::error::Error + Send + Sync>
        .map_err(|i| match i {});

    let hybrid_service = HybridService {
        web: axum_service,
        grpc: grpc_service,
    };

    // parse the listen address to a proper socket addr struct
    let addr = OPTS.listen.to_socket_addrs()?.next().unwrap();

    let server = hyper::Server::bind(&addr).serve(make_service_fn(move |_conn| {
        let hybrid_service = hybrid_service.clone();
        async { Ok::<_, axum::Error>(hybrid_service) }
    }));

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

fn setup_tracing() -> Result<(), Box<dyn std::error::Error>> {
    use opentelemetry::global;
    use opentelemetry::sdk::trace::{self, IdGenerator, Sampler};
    use tracing_opentelemetry::OpenTelemetryLayer;

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

    
    let telemetry = OpenTelemetryLayer::new(tracer);

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

// taken from https://github.com/CthulhuDen/tonic-example/blob/master/src/bin/server-hybrid.rs
#[derive(Clone)]
struct HybridService<Web, Grpc> {
    web: Web,
    grpc: Grpc,
}

impl<Web, Grpc, Error> Service<Request<Body>> for HybridService<Web, Grpc>
where
    Web: Service<Request<Body>, Response = Response<BoxBody>, Error = Error>,
    Grpc: Service<Request<Body>, Response = Response<BoxBody>, Error = Error>,
{
    type Response = Response<BoxBody>;
    type Error = Error;
    type Future = HybridFuture<Web::Future, Grpc::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(if let Err(err) = ready!(self.web.poll_ready(cx)) {
            Err(err)
        } else {
            ready!(self.web.poll_ready(cx))
        })
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        match req.headers().get(hyper::header::CONTENT_TYPE) {
            Some(ct)
                if ct.as_bytes() == b"application/grpc"
                    || ct.as_bytes().starts_with(b"application/grpc-web") =>
            {
                info!(
                    "Received grpc request, path: {}, content type {:?}",
                    req.uri(),
                    ct
                );
                HybridFuture::Grpc(self.grpc.call(req))
            }
            _ => {
                info!("Received http request, path: {}", req.uri());
                HybridFuture::Web(self.web.call(req))
            }
        }
    }
}

#[pin_project(project = HybridFutureProj)]
enum HybridFuture<WebFuture, GrpcFuture> {
    Web(#[pin] WebFuture),
    Grpc(#[pin] GrpcFuture),
}

impl<WebFuture, GrpcFuture, Output> Future for HybridFuture<WebFuture, GrpcFuture>
where
    WebFuture: Future<Output = Output>,
    GrpcFuture: Future<Output = Output>,
{
    type Output = Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        match self.project() {
            HybridFutureProj::Web(a) => a.poll(cx),
            HybridFutureProj::Grpc(b) => b.poll(cx),
        }
    }
}
