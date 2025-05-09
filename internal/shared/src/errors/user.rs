use std::fmt;

#[derive(Debug)]
pub enum UserDomainError {
    UserNotFound,
    UsernameTaken,
    InvalidEmail,
    Unauthorized,
    UsernameOrEmailTaken,
}

impl fmt::Display for UserDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserNotFound => write!(f, "User not found"),
            Self::UsernameTaken => write!(f, "Username already taken"),
            Self::InvalidEmail => write!(f, "Email is invalid"),
            Self::Unauthorized => write!(f, "Unauthorized access"),
            Self::UsernameOrEmailTaken => write!(f, "Username or email already taken"),
        }
    }
}

impl std::error::Error for UserDomainError {}
