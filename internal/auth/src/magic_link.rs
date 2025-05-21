use chrono::{DateTime, Utc};
use getset::Getters;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct MagicLink {
    email: String,
    expires_at: DateTime<Utc>,
    used: bool,
    token: String,
    attempts: u32,
}

impl MagicLink {
    pub fn new(email: String, expires_at: DateTime<Utc>, token: String) -> Self {
        Self {
            email,
            expires_at,
            used: false,
            token,
            attempts: 0,
        }
    }

    pub fn mark_as_used(&mut self) {
        self.used = true;
    }

    pub fn increment_attempts(&mut self) {
        self.attempts += 1;
    }
}
