use super::user_auth::errors::UserAuthError;
use shared::auth::jwt::JWTError;
use std::fmt;
use tracing::error;

#[derive(Debug, Clone)]
pub enum UserDomainError {
    UserNotFound,
    UsernameTaken,
    InvalidEmail,
    Unauthorized,
    UsernameOrEmailTaken,
    Authorization(UserAuthError),
    Database(String),
    Validation(String),
    Internal(String),
    InvalidTransaction,
    TransactionFailed,
    UnverifiedEmail,
    UnableToVerifyEmail,
    InvalidToken,
}

impl fmt::Display for UserDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserNotFound => write!(f, "User not found"),
            Self::UsernameTaken => write!(f, "Username already taken"),
            Self::InvalidEmail => write!(f, "Email is invalid"),
            Self::Unauthorized => write!(f, "Unauthorized access"),
            Self::UsernameOrEmailTaken => write!(f, "Username or email already taken"),
            Self::Authorization(err) => write!(f, "{}", err),
            Self::Database(msg) => write!(f, "{}", msg),
            Self::Validation(msg) => write!(f, "{}", msg),
            Self::Internal(msg) => write!(f, "{}", msg),
            Self::InvalidTransaction => write!(f, "Invalid transaction"),
            Self::TransactionFailed => write!(f, "Transaction failed"),
            Self::UnverifiedEmail => write!(f, "Email is not verified"),
            Self::UnableToVerifyEmail => write!(f, "Unable to verify email address"),
            Self::InvalidToken => write!(f, "Invalid token"),
        }
    }
}

impl std::error::Error for UserDomainError {}

impl From<UserAuthError> for UserDomainError {
    fn from(err: UserAuthError) -> Self {
        Self::Authorization(err)
    }
}

impl From<mongodb::error::Error> for UserDomainError {
    fn from(err: mongodb::error::Error) -> Self {
        error!("Mongodb Error: {:#?}", err);
        Self::Database("Database error".to_string()) // TODO: Return a generic database error instead of the specific error
    }
}

impl From<validator::ValidationErrors> for UserDomainError {
    fn from(err: validator::ValidationErrors) -> Self {
        let json =
            serde_json::to_string(&err).unwrap_or_else(|_| format!("{{\"error\": \"{}\"}}", err));
        Self::Validation(json)
    }
}

impl From<JWTError> for UserDomainError {
    fn from(err: JWTError) -> Self {
        error!("JWT Error: {:#?}", err);
        Self::InvalidToken
    }
}
