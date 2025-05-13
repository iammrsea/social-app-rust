use chrono;

pub mod non_empty_string;

pub type Date = chrono::DateTime<chrono::Utc>;
pub type Utc = chrono::Utc;
pub type Duration = chrono::Duration;
pub type DateTime<T> = chrono::DateTime<T>;
pub type AppResult<T> = Result<T, crate::errors::app::AppError>;

// fn get_current_time() -> Date {
//     DateTime::parse_from_rfc3339(s)
// }
