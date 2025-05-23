use crate::domain::user_read_model::{Ban, BanType as DomainBanType, UserReadModel};
use async_graphql::{Enum, Object};
use shared::types::graphql_scalars::DateTimeScalar;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Enum)]
#[graphql(remote = "shared::guards::roles::UserRole")]
enum UserRole {
    Admin,
    Moderator,
    Regular,
    Guest,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Enum)]
enum BanType {
    Definite,
    Indefinite,
}

impl From<DomainBanType> for BanType {
    fn from(ban_type: DomainBanType) -> Self {
        match ban_type {
            DomainBanType::Definite { .. } => BanType::Definite,
            DomainBanType::Indefinite => BanType::Indefinite,
        }
    }
}

#[Object(name = "User")]
impl UserReadModel {
    async fn id(&self) -> String {
        self.id.to_owned()
    }
    async fn username(&self) -> String {
        self.username.to_owned()
    }
    async fn email(&self) -> String {
        self.email.to_owned()
    }
    async fn role(&self) -> UserRole {
        self.role.to_owned().into()
    }
    async fn badges(&self) -> Vec<String> {
        self.badges.to_owned()
    }
    async fn created_at(&self) -> DateTimeScalar {
        self.created_at.into()
    }
    async fn updated_at(&self) -> DateTimeScalar {
        self.updated_at.into()
    }
    async fn ban_status(&self) -> Option<Ban> {
        self.ban_status.to_owned()
    }
}
#[Object]
impl Ban {
    async fn is_banned(&self) -> bool {
        self.is_banned
    }
    async fn reason(&self) -> String {
        self.reason.to_owned()
    }
    async fn banned_at(&self) -> DateTimeScalar {
        self.banned_at.into()
    }
    async fn ban_type(&self) -> BanType {
        self.ban_type.to_owned().into()
    }
    async fn from(&self) -> Option<DateTimeScalar> {
        match self.ban_type {
            DomainBanType::Definite { from, .. } => Some(from.into()),
            DomainBanType::Indefinite => None,
        }
    }
    async fn to(&self) -> Option<DateTimeScalar> {
        match self.ban_type {
            DomainBanType::Definite { to, .. } => Some(to.into()),
            DomainBanType::Indefinite => None,
        }
    }
}
