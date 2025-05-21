use std::fmt;

#[derive(Debug, Clone)]
pub enum AuthError {
    OtpNotFound,
    InvalidToken,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OtpNotFound => write!(f, "Otp not found"),
            Self::InvalidToken => write!(f, "Invalid token"),
        }
    }
}

impl std::error::Error for AuthError {}
