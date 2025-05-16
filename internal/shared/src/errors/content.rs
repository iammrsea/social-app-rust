use std::fmt;

#[derive(Debug, Clone)]
pub enum ContentDomainError {
    PostNotFound,
    InvalidTitle,
}

impl fmt::Display for ContentDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PostNotFound => write!(f, "Post not found"),
            Self::InvalidTitle => write!(f, "Title is invalid"),
        }
    }
}

impl std::error::Error for ContentDomainError {}
