use std::fmt;
use user::domain::errors::UserDomainError;

#[derive(Debug, Clone)]
pub enum AppError {
    User(UserDomainError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for AppError {}

impl From<UserDomainError> for AppError {
    fn from(err: UserDomainError) -> Self {
        Self::User(err)
    }
}

pub type AppResult<T> = Result<T, AppError>;
