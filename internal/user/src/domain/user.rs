use serde::{Deserialize, Serialize};

use shared::guards::rbac::roles::UserRole;
use shared::types::non_empty_string::NonEmptyString;
use shared::types::{Date, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BanType {
    Definite { from: Date, to: Date },
    Indefinite,
}

#[derive(Debug, PartialEq)]
pub struct Ban {
    is_banned: bool,
    reason: String,
    banned_at: Date,
    ban_type: BanType,
}

#[derive(Debug)]
pub struct User {
    id: String,
    email: String,
    username: String,
    role: UserRole,
    joined_at: Date,
    ban_status: Option<Ban>,
    updated_at: Date,
    badges: Vec<String>,
}

impl User {
    pub fn new(id: String, email: String, username: String, role: UserRole) -> Self {
        Self {
            id,
            email,
            username,
            role,
            joined_at: Utc::now(),
            ban_status: None,
            updated_at: Utc::now(),
            badges: vec![],
        }
    }
    pub fn ban(&mut self, reason: NonEmptyString, ban_type: BanType) {
        if let Some(ban) = self.ban_status.as_mut() {
            ban.is_banned = true;
            ban.reason = reason.into();
            ban.banned_at = Utc::now();
            ban.ban_type = ban_type;
        } else {
            self.ban_status = Some(Ban {
                banned_at: Utc::now(),
                is_banned: true,
                ban_type,
                reason: reason.into(),
            })
        }
        self.updated_at = Utc::now();
    }

    pub fn unban(&mut self) {
        self.ban_status = None;
        self.updated_at = Utc::now();
    }
    pub fn change_username(&mut self, new_username: NonEmptyString) {
        self.username = new_username.into();
        self.updated_at = Utc::now();
    }
    pub fn award_badge(&mut self, badge: NonEmptyString) {
        self.badges.push(badge.into());
    }
    pub fn revoke_badge(&mut self, badge: NonEmptyString) {
        self.badges.retain(|b| *b != *badge);
        self.updated_at = Utc::now();
    }
    pub fn make_moderator(&mut self) {
        self.role = UserRole::Moderator
    }
    pub fn make_regular(&mut self) {
        self.role = UserRole::Regular
    }
}

/// Implement getters;
impl User {
    pub fn joined_at(&self) -> &Date {
        &self.joined_at
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn role(&self) -> &UserRole {
        &self.role
    }
    pub fn updated_at(&self) -> &Date {
        &self.updated_at
    }
    pub fn ban_status(&self) -> Option<&Ban> {
        self.ban_status.as_ref()
    }
    pub fn badges(&self) -> &Vec<String> {
        &self.badges
    }
    pub fn is_moderator(&self) -> bool {
        self.role == UserRole::Moderator
    }
    pub fn is_regular(&self) -> bool {
        self.role == UserRole::Regular
    }
    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }
}

impl Ban {
    pub fn is_banned(&self) -> bool {
        self.is_banned
    }
    pub fn reason_for_ban(&self) -> &str {
        &self.reason
    }
    pub fn ban_type(&self) -> &BanType {
        &self.ban_type
    }
    pub fn banned_at(&self) -> &Date {
        &self.banned_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::types::Duration;
    use uuid::Uuid;

    #[test]
    fn ban_user_definitely() {
        let mut user = User::new(
            Uuid::new_v4().into(),
            "johndoe@gmail.com".into(),
            "johndoe".into(),
            UserRole::Regular,
        );
        let reason = NonEmptyString::new("abuse".into()).unwrap();
        let ban_type = BanType::Definite {
            from: Utc::now(),
            to: Utc::now() + Duration::days(30),
        };
        user.ban(reason, ban_type.clone());
        let ban_status = user.ban_status().unwrap();
        assert!(ban_status.is_banned(), "expected user to be banned");
        assert_eq!(
            &ban_type,
            ban_status.ban_type(),
            "expected ban type: {:#?}, got ban type: {:#?}",
            ban_type,
            ban_status.ban_type()
        )
    }

    #[test]
    fn ban_user_indefinitely() {
        let mut user = User::new(
            Uuid::new_v4().into(),
            "johndoe@gmail.com".into(),
            "johndoe".into(),
            UserRole::Regular,
        );
        let reason = NonEmptyString::new("abuse".into()).unwrap();
        let ban_type = BanType::Indefinite;
        user.ban(reason, ban_type.clone());
        let ban_status = user.ban_status().unwrap();
        assert!(ban_status.is_banned(), "expected user to be banned");
        assert_eq!(
            &ban_type,
            ban_status.ban_type(),
            "expected ban type: {:#?}, got ban type: {:#?}",
            ban_type,
            ban_status.ban_type()
        )
    }

    #[test]
    fn unban_user() {
        let mut user = User::new(
            Uuid::new_v4().into(),
            "johndoe@gmail.com".into(),
            "johndoe".into(),
            UserRole::Regular,
        );
        let reason = NonEmptyString::new("abuse".into()).unwrap();
        let ban_type = BanType::Indefinite;
        user.ban(reason, ban_type);
        user.unban();

        assert_eq!(
            None,
            user.ban_status(),
            "expected user ban status: {:#?}, got user ban status: {:#?}",
            None as Option<Ban>,
            user.ban_status()
        );
    }

    #[test]
    fn change_username() {
        let mut user = User::new(
            Uuid::new_v4().into(),
            "johndoe@gmail.com".into(),
            "johndoe".into(),
            UserRole::Regular,
        );
        let new_username = NonEmptyString::new("johndoe123".into()).unwrap();
        user.change_username(new_username.clone());
        assert_eq!(
            new_username,
            user.username(),
            "expected username: {}, got username: {}",
            new_username,
            user.username()
        )
    }

    #[test]
    fn award_badge() {
        let mut user = User::new(
            Uuid::new_v4().into(),
            "johndoe@gmail.com".into(),
            "johndoe".into(),
            UserRole::Regular,
        );
        let badge = NonEmptyString::new("5-star".into()).unwrap();
        user.award_badge(badge);
        assert_eq!(
            1,
            user.badges().len(),
            "expected badges: {}, got badges: {}",
            1,
            user.badges().len()
        )
    }

    #[test]
    fn revoke_badge() {
        let mut user = User::new(
            Uuid::new_v4().into(),
            "johndoe@gmail.com".into(),
            "johndoe".into(),
            UserRole::Regular,
        );
        let badge = NonEmptyString::new("5-star".into()).unwrap();
        user.award_badge(badge.clone());
        user.revoke_badge(badge);
        assert_eq!(
            0,
            user.badges().len(),
            "expected badges: {}, got badges: {}",
            0,
            user.badges().len()
        )
    }

    #[test]
    fn make_moderator() {
        let mut user = User::new(
            Uuid::new_v4().into(),
            "johndoe@gmail.com".into(),
            "johndoe".into(),
            UserRole::Regular,
        );
        user.make_moderator();
        assert_eq!(
            &UserRole::Moderator,
            user.role(),
            "expected role: {:#?}, got role: {:#?}",
            UserRole::Moderator,
            user.role()
        )
    }

    #[test]
    fn make_regular() {
        let mut user = User::new(
            Uuid::new_v4().into(),
            "johndoe@gmail.com".into(),
            "johndoe".into(),
            UserRole::Regular,
        );
        user.make_regular();
        assert_eq!(
            &UserRole::Regular,
            user.role(),
            "expected role: {:#?}, got role: {:#?}",
            UserRole::Regular,
            user.role()
        )
    }
}
