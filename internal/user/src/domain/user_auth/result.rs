use super::errors;

pub type UserAuthResult<T> = std::result::Result<T, errors::UserAuthError>;
