#[derive(Debug)]
pub enum Error {
    Parse(String),
    Validate(String),
    NotFound,
}

impl std::error::Error for Error {}

impl From<tonic::metadata::errors::ToStrError> for Error {
    fn from(item: tonic::metadata::errors::ToStrError) -> Self {
        Error::Parse(format!("{}", item))
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(item: jsonwebtoken::errors::Error) -> Self {
        Error::Validate(format!("{}", item))
    }
}

impl From<sqlx::types::uuid::Error> for Error {
    fn from(item: sqlx::types::uuid::Error) -> Self {
        Error::Parse(format!("{}", item))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Parse(cause) => write!(f, "failed to parse: {}", cause),
            Error::Validate(cause) => write!(f, "failed to validate: {}", cause),
            Error::NotFound => write!(f, "no token found"),
        }
    }
}

impl From<Error> for tonic::Status {
    fn from(item: Error) -> Self {
        tonic::Status::unauthenticated(format!("failed to authenticate request: {}", item))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(item: std::str::Utf8Error) -> Self {
        Error::Parse(format!("failed to parse token from headers: {}", item))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(item: std::num::ParseIntError) -> Self {
        Error::Parse(format!("failed to parse token from headers: {}", item))
    }
}
