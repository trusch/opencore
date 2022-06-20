use tracing::error;

#[derive(Debug, Clone)]
pub enum Error {
    Database(String),
    InvalidArgument(String),
    Send(String),
    InvalidFencingToken,
    NotFound,
    Forbidden,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Database(cause) => write!(f, "failed to interact with the database: {}", cause),
            Error::InvalidArgument(cause) => write!(f, "failed to handle an argument: {}", cause),
            Error::Send(cause) => write!(f, "failed to send data: {}", cause),
            Error::NotFound => write!(f, "not found"),
            Error::Forbidden => write!(f, "forbidden"),
            Error::InvalidFencingToken => write!(f, "invalid fencing token"),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(item: sqlx::Error) -> Self {
        error!("{}", item);
        match item {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::Database(format!("failed to interact with the database: {}", item)),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(item: std::io::Error) -> Self {
        error!("{}", item);
        Error::Database(format!("failed to interact with the filesystem: {}", item))
    }
}

impl From<serde_json::Error> for Error {
    fn from(item: serde_json::Error) -> Self {
        error!("{}", item);
        Error::InvalidArgument(format!("failed to encode/decode data: {}", item))
    }
}

impl From<sqlx::types::uuid::Error> for Error {
    fn from(item: sqlx::types::uuid::Error) -> Self {
        error!("{}", item);
        Error::InvalidArgument(format!("failed to parse uuid: {}", item))
    }
}

impl<T> From<tokio::sync::broadcast::error::SendError<T>> for Error {
    fn from(item: tokio::sync::broadcast::error::SendError<T>) -> Self {
        error!("{}", item);
        Error::Send(format!("failed to send data: {}", item))
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(item: tokio::sync::mpsc::error::SendError<T>) -> Self {
        error!("{}", item);
        Error::Send(format!("failed to send data: {}", item))
    }
}

impl<'a> From<jsonschema::ValidationError<'a>> for Error {
    fn from(item: jsonschema::ValidationError<'a>) -> Self {
        error!("{}", item);
        Error::InvalidArgument(format!("failed to compile json schema: {}", item))
    }
}

impl From<prost::DecodeError> for Error {
    fn from(item: prost::DecodeError) -> Self {
        error!("{}", item);
        Error::Database(format!("failed to decode protobuf message: {}", item))
    }
}

impl From<crate::token::error::Error> for Error {
    fn from(item: crate::token::error::Error) -> Self {
        error!("{}", item);
        Error::InvalidArgument(format!("failed to handle claim data: {}", item))
    }
}
