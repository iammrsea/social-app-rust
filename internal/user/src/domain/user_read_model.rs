use serde::{Deserialize, Serialize};

use super::user::BanType;
use shared::guards::rbac::roles::UserRole;
use shared::types::Date;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ban {
    pub is_banned: bool,
    pub reason: String,
    pub banned_at: Date,
    pub ban_type: BanType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReadModel {
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub id: String,
    pub badges: Vec<String>,
    pub created_at: Date,
    pub updated_at: Date,
    pub ban_status: Option<Ban>,
}
