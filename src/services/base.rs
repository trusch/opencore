use sqlx::types::Uuid;
use tonic::Status;

pub trait BaseService {
    #[tracing::instrument]
    fn parse_uuid(id: &str) -> Result<Uuid, Status> {
        match Uuid::parse_str(id) {
            Ok(id) => Ok(id),
            Err(err) => {
                return Err(Status::invalid_argument(format!(
                    "failed to parse uuid: {}",
                    err
                )));
            }
        }
    }

    #[tracing::instrument]
    fn parse_json(doc: &str) -> Result<serde_json::Value, Status> {
        match serde_json::from_str(doc) {
            Ok(data) => Ok(data),
            Err(err) => {
                return Err(Status::invalid_argument(format!(
                    "failed to parse json data: {}",
                    err
                )));
            }
        }
    }
}
