pub mod graphql_scalars;
pub mod non_empty_string;

pub type AppResult<T> = Result<T, crate::errors::app::AppError>;
