use serde::{Deserialize, Serialize};

use super::user::BanType;
use shared::guards::roles::UserRole;
use shared::types::{Date, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ban {
    pub is_banned: bool,
    pub reason: String,
    pub banned_at: Date,
    pub ban_type: BanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReadModel {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub badges: Vec<String>,
    pub created_at: Date,
    pub updated_at: Date,
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
