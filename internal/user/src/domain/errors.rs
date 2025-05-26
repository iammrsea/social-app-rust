use auth::errors::AuthError;
use std::fmt;
use tracing::error;

#[derive(Debug, Clone)]
pub enum UserDomainError {
    UserNotFound,
    UsernameTaken,
    InvalidEmail,
    Unauthorized,
    UsernameOrEmailTaken,
    Authorization(AuthError),
    Database(String),
    Validation(String),
    Internal(String),
    InvalidTransaction,
    TransactionFailed,
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
        }
    }
}

impl std::error::Error for UserDomainError {}

impl From<AuthError> for UserDomainError {
    fn from(err: AuthError) -> Self {
        Self::Authorization(err)
    }
}

impl From<mongodb::error::Error> for UserDomainError {
    fn from(err: mongodb::error::Error) -> Self {
        error!("Mongodb Error: {:#?}", err);
        Self::Database(err.to_string()) // TODO: Return a generic database error instead of the specific error
    }
}

impl From<validator::ValidationErrors> for UserDomainError {
    fn from(err: validator::ValidationErrors) -> Self {
        let json =
            serde_json::to_string(&err).unwrap_or_else(|_| format!("{{\"error\": \"{}\"}}", err));
        Self::Validation(json)
    }
}
