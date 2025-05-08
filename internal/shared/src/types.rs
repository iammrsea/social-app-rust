use chrono;

pub mod non_empty_string;

pub type Date = chrono::DateTime<chrono::Utc>;
pub type Utc = chrono::Utc;
pub type Duration = chrono::Duration;
pub type AppResult<T> = Result<T, crate::errors::app::AppError>;
