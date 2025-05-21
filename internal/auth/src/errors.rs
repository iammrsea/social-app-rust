use std::fmt;
use tracing::error;

#[derive(Debug, Clone)]
pub enum AuthError {
    OtpNotFound,
    InvalidToken,
    OtpAlreadyUsed,
    OtpExpired,
    TooManyAttempts,
    InvalidOtp,
    Database,
    Validation(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OtpNotFound => write!(f, "No otp found for this email"),
            Self::InvalidToken => write!(f, "Invalid token"),
            Self::Database => write!(f, "Database error"),
            Self::OtpAlreadyUsed => write!(f, "Otp already used"),
            Self::OtpExpired => write!(f, "Otp expired"),
            Self::TooManyAttempts => write!(f, "Too many attempts"),
            Self::InvalidOtp => write!(f, "Invalid otp"),
            Self::Validation(json) => write!(f, "{}", json),
        }
    }
}

impl std::error::Error for AuthError {}

impl From<mongodb::error::Error> for AuthError {
    fn from(err: mongodb::error::Error) -> Self {
        error!("Mongodb Error: {:#?}", err);
        Self::Database
    }
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        error!("JWT Error: {:#?}", err);
        Self::InvalidToken
    }
}

impl From<validator::ValidationErrors> for AuthError {
    fn from(err: validator::ValidationErrors) -> Self {
        let json =
            serde_json::to_string(&err).unwrap_or_else(|_| format!("{{\"error\": \"{}\"}}", err));
        Self::Validation(json)
    }
}
