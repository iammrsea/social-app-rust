use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
use shared::guards::roles::UserRole;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BanType {
    Definite {
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    },
    Indefinite,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ban {
    pub is_banned: bool,
    pub reason: String,
    pub banned_at: DateTime<Utc>,
    pub ban_type: BanType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserReadModel {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub badges: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub ban_status: Option<Ban>,
}

impl UserReadModel {
    pub fn new_test_user_read_model() -> Self {
        Self {
            id: "user_id".into(),
            username: "test".into(),
            email: "test@gmail.com".into(),
            role: UserRole::Regular,
            badges: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            ban_status: None,
        }
    }
}
