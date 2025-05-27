use std::fmt;
use tracing::error;

#[derive(Debug, Clone, PartialEq)]
pub enum UserAuthError {
    OtpNotFound,
    InvalidToken,
    OtpAlreadyUsed,
    OtpExpired,
    TooManyAttempts,
    InvalidOtp,
    Database,
    InvalidTransaction,
    MissMatchOtp,
}

impl fmt::Display for UserAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OtpNotFound => write!(f, "No otp found for this email"),
            Self::InvalidToken => write!(f, "Invalid token"),
            Self::Database => write!(f, "Database error"),
            Self::OtpAlreadyUsed => write!(f, "Otp already used"),
            Self::OtpExpired => write!(f, "Otp expired"),
            Self::TooManyAttempts => write!(f, "Too many attempts"),
            Self::InvalidOtp => write!(f, "Invalid otp"),
            Self::InvalidTransaction => write!(f, "Invalid transaction"),
            Self::MissMatchOtp => write!(f, "Otp does not match"),
        }
    }
}

impl std::error::Error for UserAuthError {}

impl From<mongodb::error::Error> for UserAuthError {
    fn from(err: mongodb::error::Error) -> Self {
        error!("Mongodb Error: {:#?}", err);
        Self::Database
    }
}

impl From<jsonwebtoken::errors::Error> for UserAuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        error!("JWT Error: {:#?}", err);
        Self::InvalidToken
    }
}
