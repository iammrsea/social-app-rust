pub mod auth_service;
mod auth_utils;
pub mod errors;
pub mod jwt;
mod magic_link;
mod otp;

pub use otp::{mongoimpl, otp_respository};

pub type AuthResult<T> = std::result::Result<T, errors::AuthError>;
