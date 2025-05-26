use super::errors::UserDomainError;

pub type UserDomainResult<T> = Result<T, UserDomainError>;
