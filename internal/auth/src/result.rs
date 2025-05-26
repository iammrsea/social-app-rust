use crate::errors;

pub type AuthResult<T> = std::result::Result<T, errors::AuthError>;
