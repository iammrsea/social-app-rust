use crate::errors::{content::ContentDomainError, user::UserDomainError};
use std::fmt;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    User(UserDomainError),
    Content(ContentDomainError),
    Database(String),
    Internal(String),
    NonEmptyString,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User(err) => write!(f, "User error: {}", err),
            Self::Content(err) => write!(f, "Content error: {}", err),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
            Self::NonEmptyString => write!(f, "Empty string is not allowed"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<UserDomainError> for AppError {
    fn from(err: UserDomainError) -> Self {
        Self::User(err)
    }
}

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
