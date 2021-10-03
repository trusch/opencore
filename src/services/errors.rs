use crate::managers::errors::Error;

impl From<Error> for tonic::Status {
    fn from(item: Error) -> Self {
        match item {
            Error::Database(cause) => tonic::Status::internal(cause),
            Error::Send(cause) => tonic::Status::internal(cause),
            Error::InvalidArgument(cause) => tonic::Status::invalid_argument(cause),
            Error::NotFound => tonic::Status::not_found("not found"),
            Error::Forbidden => tonic::Status::permission_denied("you have no right to do this"),
            Error::InvalidFencingToken => {
                tonic::Status::resource_exhausted("invalid fencing token")
            }
        }
    }
}
