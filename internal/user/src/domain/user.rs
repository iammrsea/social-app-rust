use serde::{Deserialize, Serialize};
use uuid::Uuid;

use chrono::{DateTime, Utc};
use shared::guards::roles::UserRole;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BanType {
    Definite {
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    },
    Indefinite,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EmailStatus {
    Verified,
    Unverified,
}
impl std::fmt::Display for EmailStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailStatus::Verified => write!(f, "Verified"),
            EmailStatus::Unverified => write!(f, "Unverified"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ban {
    is_banned: bool,
    reason: String,
    banned_at: DateTime<Utc>,
    ban_type: BanType,
}

#[derive(Debug, Clone)]
pub struct User {
    id: String,
    email: String,
    username: String,
    role: UserRole,
    joined_at: DateTime<Utc>,
    ban_status: Option<Ban>,
    updated_at: DateTime<Utc>,
    badges: Vec<String>,
    email_status: EmailStatus,
}

impl User {
    pub fn new(email: String, username: String, role: UserRole) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            email: email,
            username: username,
            role,
            joined_at: Utc::now(),
            ban_status: None,
            updated_at: Utc::now(),
            badges: vec![],
            email_status: EmailStatus::Unverified,
        }
    }
    pub fn new_with_all_fields(
        id: String,
        email: String,
        username: String,
        role: UserRole,
        joined_at: DateTime<Utc>,
        ban_status: Option<Ban>,
        updated_at: DateTime<Utc>,
        badges: Vec<String>,
        email_status: EmailStatus,
    ) -> Self {
        Self {
            id,
            email: email,
            username: username,
            role,
            joined_at,
            ban_status,
            updated_at,
            badges,
            email_status,
        }
    }
    pub fn ban(&mut self, reason: String, ban_type: BanType) {
        if let Some(ban) = self.ban_status.as_mut() {
            ban.is_banned = true;
            ban.reason = reason;
            ban.banned_at = Utc::now();
            ban.ban_type = ban_type;
        } else {
            self.ban_status = Some(Ban {
                banned_at: Utc::now(),
                is_banned: true,
                ban_type,
                reason: reason,
            })
        }
        self.updated_at = Utc::now();
    }

    pub fn unban(&mut self) {
        self.ban_status = None;
        self.updated_at = Utc::now();
    }
    pub fn change_username(&mut self, new_username: String) {
        self.username = new_username;
        self.updated_at = Utc::now();
    }
    pub fn award_badge(&mut self, badge: String) {
        self.badges.push(badge);
    }
    pub fn revoke_badge(&mut self, badge: String) {
        self.badges.retain(|b| *b != badge);
        self.updated_at = Utc::now();
    }
    pub fn make_moderator(&mut self) {
        self.role = UserRole::Moderator
    }
    pub fn make_regular(&mut self) {
        self.role = UserRole::Regular
    }
    pub fn new_test_user(role: Option<UserRole>) -> User {
        let role = role.unwrap_or(UserRole::Regular);
        User {
            id: User::test_user_id(),
            email: "johndoe@gmail.com".into(),
            username: "johndoe".into(),
            role,
            joined_at: Utc::now(),
            ban_status: None,
            updated_at: Utc::now(),
            badges: vec![],
            email_status: EmailStatus::Verified,
        }
    }
    pub fn test_user_id() -> String {
        "user-id123".into()
    }
    pub fn set_email_status(&mut self, email_status: EmailStatus) {
        self.email_status = email_status;
        self.updated_at = Utc::now();
    }
}

/// Implement getters;
impl User {
    pub fn joined_at(&self) -> &DateTime<Utc> {
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
    pub fn updated_at(&self) -> &DateTime<Utc> {
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
    pub fn email_status(&self) -> &EmailStatus {
        &self.email_status
    }
}

impl Ban {
    pub fn new(
        reason: String,
        is_banned: bool,
        banned_at: DateTime<Utc>,
        ban_type: BanType,
    ) -> Self {
        Self {
            is_banned,
            reason: reason,
            banned_at,
            ban_type,
        }
    }
    pub fn is_banned(&self) -> bool {
        self.is_banned
    }
    pub fn reason_for_ban(&self) -> &str {
        &self.reason
    }
    pub fn ban_type(&self) -> &BanType {
        &self.ban_type
    }
    pub fn banned_at(&self) -> &DateTime<Utc> {
        &self.banned_at
    }
}

#[cfg(test)]
mod tests {
    use super::UserRole::Moderator;
    use super::*;
    use chrono::Duration;

    #[test]
    fn ban_user_definitely() {
        let mut user = User::new_test_user(None);
        let ban_type = BanType::Definite {
            from: Utc::now(),
            to: Utc::now() + Duration::days(30),
        };
        user.ban("abuse".into(), ban_type.clone());
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
        let mut user = User::new_test_user(None);
        let ban_type = BanType::Indefinite;
        user.ban("abuse".into(), ban_type.clone());
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
        let mut user = User::new_test_user(None);
        let ban_type = BanType::Indefinite;
        user.ban("abuse".into(), ban_type);
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
        let mut user = User::new_test_user(None);
        let new_username = "johndoe123".to_string();
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
        let mut user = User::new_test_user(None);
        user.award_badge("5-star".into());
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
        let mut user = User::new_test_user(None);
        let badge = "5-star".to_string();
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
        let mut user = User::new_test_user(None);
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
        let mut user = User::new_test_user(Some(Moderator));
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
