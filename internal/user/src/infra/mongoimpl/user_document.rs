use bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};
use shared::types::non_empty_string::NonEmptyString;

use crate::domain::user::Ban as BanDomain;
use crate::domain::user::BanType as BanTypeDomain;
use crate::domain::user::User;
use crate::domain::user_read_model::Ban as BanReadModel;
use crate::domain::user_read_model::BanType as BanTypeReadModel;
use crate::domain::user_read_model::UserReadModel;
use chrono::DateTime;
use chrono::Utc;
use shared::guards::roles::UserRole;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BanType {
    Definite {
        #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
        from: DateTime<Utc>,
        #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
        to: DateTime<Utc>,
    },
    Indefinite,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Ban {
    pub is_banned: bool,
    pub reason: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub banned_at: DateTime<Utc>,
    pub ban_type: BanType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserDocument {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub badges: Vec<String>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    pub ban_status: Option<Ban>,
}

impl From<UserDocument> for UserReadModel {
    fn from(value: UserDocument) -> Self {
        UserReadModel {
            id: value.id,
            username: value.username,
            email: value.email,
            role: value.role,
            badges: value.badges,
            created_at: value.created_at,
            updated_at: value.updated_at,
            ban_status: value.ban_status.map(|b| BanReadModel {
                is_banned: b.is_banned,
                reason: b.reason,
                banned_at: b.banned_at,
                ban_type: match b.ban_type {
                    BanType::Definite { from, to } => BanTypeReadModel::Definite { from, to },
                    BanType::Indefinite => BanTypeReadModel::Indefinite,
                },
            }),
        }
    }
}
impl Into<UserDocument> for UserReadModel {
    fn into(self) -> UserDocument {
        UserDocument {
            id: self.id,
            username: self.username,
            email: self.email,
            role: self.role,
            badges: self.badges,
            created_at: self.created_at,
            updated_at: self.updated_at,
            ban_status: self.ban_status.map(|b| Ban {
                is_banned: b.is_banned,
                reason: b.reason,
                banned_at: b.banned_at,
                ban_type: match b.ban_type {
                    BanTypeReadModel::Definite { from, to } => BanType::Definite { from, to },
                    BanTypeReadModel::Indefinite => BanType::Indefinite,
                },
            }),
        }
    }
}

impl From<UserDocument> for User {
    fn from(value: UserDocument) -> Self {
        let ban_status = value.ban_status.as_ref().map(|b| {
            let ban_type = match b.ban_type {
                BanType::Definite { from, to } => BanTypeDomain::Definite { from, to },
                BanType::Indefinite => BanTypeDomain::Indefinite,
            };
            BanDomain::new(
                NonEmptyString::new(b.reason.clone()).unwrap(),
                b.is_banned,
                b.banned_at,
                ban_type,
            )
        });

        User::new_with_all_fields(
            value.id,
            NonEmptyString::new(value.email).unwrap(),
            NonEmptyString::new(value.username).unwrap(),
            value.role,
            value.created_at,
            ban_status,
            value.updated_at,
            value.badges,
        )
    }
}
impl Into<UserDocument> for User {
    fn into(self) -> UserDocument {
        let ban_status = self.ban_status().map(|b| {
            let ban_type = match b.ban_type() {
                BanTypeDomain::Definite { from, to } => BanType::Definite {
                    from: truncate_chrono(from),
                    to: truncate_chrono(to),
                },
                BanTypeDomain::Indefinite => BanType::Indefinite,
            };
            Ban {
                is_banned: b.is_banned(),
                reason: b.reason_for_ban().to_string(),
                banned_at: truncate_chrono(b.banned_at()),
                ban_type,
            }
        });
        UserDocument {
            id: self.id().to_string(),
            username: self.username().to_string(),
            email: self.email().to_string(),
            role: self.role().to_owned(),
            badges: self.badges().to_owned(),
            created_at: truncate_chrono(self.joined_at()),
            updated_at: truncate_chrono(self.updated_at()),
            ban_status,
        }
    }
}

fn truncate_chrono(date: &DateTime<Utc>) -> DateTime<Utc> {
    BsonDateTime::from_millis(date.to_owned().timestamp_millis()).to_chrono()
}
