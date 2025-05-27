use crate::errors::content::ContentDomainError;
use std::fmt;
use tracing::error;

#[derive(Debug, Clone)]
pub enum AppError {
    Content(ContentDomainError),
    Database(String),
    Internal(String),
    NonEmptyString,
    Base64(String),
    Validation(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Content(err) => write!(f, "Content error: {}", err),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
            Self::NonEmptyString => write!(f, "Empty string is not allowed"),
            Self::Base64(msg) => write!(f, "Base64 error: {}", msg),
            Self::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<ContentDomainError> for AppError {
    fn from(err: ContentDomainError) -> Self {
        Self::Content(err)
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        error!("Mongodb Error: {:#?}", err);
        Self::Database(err.to_string())
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(err: base64::DecodeError) -> Self {
        Self::Base64(err.to_string())
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        let json =
            serde_json::to_string(&err).unwrap_or_else(|_| format!("{{\"error\": \"{}\"}}", err));
        Self::Validation(json)
    }
}
