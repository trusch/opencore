use crate::api;
use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use api::catalog::events_server::Events;
use api::catalog::{Event, PublishRequest, SubscribeRequest};

use crate::managers;
use crate::token;

use super::base::BaseService;

#[derive(Debug)]
pub struct Service {
    validator: Arc<token::Validator>,
    manager: Arc<managers::events::Manager>,
}

impl BaseService for Service {}

impl Service {
    pub fn new(
        validator: Arc<token::Validator>,
        manager: Arc<managers::events::Manager>,
    ) -> Result<Service, sqlx::Error> {
        let res = Service { validator, manager };
        Ok(res)
    }
}

#[tonic::async_trait]
impl Events for Service {
    #[tracing::instrument(name = "svc::events::publish", skip(self))]
    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<Event>, Status> {
        let r = request.get_ref();
        let claims = self.validator.get_access_token_claims(&request)?;
        let data = Self::parse_json(&r.data)?;
        let resource_id = Self::parse_uuid(&r.resource_id)?;
        let evt = self
            .manager
            .publish(
                &claims,
                &resource_id,
                &r.resource_kind,
                &r.labels,
                r.event_type.into(),
                &data,
            )
            .await?;
        Ok(Response::new(evt))
    }

    type SubscribeStream =
        Pin<Box<dyn Stream<Item = Result<Event, Status>> + Send + Sync + 'static>>;

    #[tracing::instrument(name = "svc::events::subscribe", skip(self))]
    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let claims = self.validator.get_access_token_claims(&request)?;
        let mut filters = vec![];

        use crate::api::catalog::EventType;
        use managers::events::SubscribeFilter;

        let resource_id = &request.get_ref().resource_id;
        if !resource_id.is_empty() {
            filters.push(SubscribeFilter::Resource(resource_id.to_string()));
        }

        let kind = &request.get_ref().resource_kind;
        if !kind.is_empty() {
            filters.push(SubscribeFilter::Kind(kind.to_string()));
        }

        let event_type = EventType::from(request.get_ref().event_type);
        if event_type != EventType::None {
            log::info!("filter by event type {:?}", event_type);
            filters.push(SubscribeFilter::Type(event_type));
        }

        let stream = self.manager.subscribe(&claims, &filters).await?;
        Ok(Response::new(stream))
    }
}

impl From<i32> for api::catalog::EventType {
    fn from(item: i32) -> Self {
        match item {
            1 => api::catalog::EventType::Create,
            2 => api::catalog::EventType::Update,
            3 => api::catalog::EventType::Delete,
            _ => api::catalog::EventType::None,
        }
    }
}
