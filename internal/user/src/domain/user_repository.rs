use std::pin::Pin;

use async_trait::async_trait;
use mongodb::ClientSession;

use super::errors::UserDomainResult;
use super::user::{EmailStatus, User};

pub type F = Box<dyn FnOnce(&mut User) + Send>;
pub type Ftx = Box<
    dyn FnOnce(&mut ClientSession) -> Pin<Box<dyn Future<Output = UserDomainResult<()>> + Send>>
        + Send,
>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_account(&self, user: User) -> UserDomainResult<()>;

    async fn make_moderator(&self, user_id: &str, update_fn: F) -> UserDomainResult<()>;

    async fn change_username(&self, user_id: &str, update_fn: F) -> UserDomainResult<()>;

    async fn award_badge(&self, user_id: &str, update_fn: F) -> UserDomainResult<()>;

    async fn revoke_badge(&self, user_id: &str, update_fn: F) -> UserDomainResult<()>;

    async fn ban_user(&self, user_id: &str, update_fn: F) -> UserDomainResult<()>;

    async fn unban_user(&self, user_id: &str, update_fn: F) -> UserDomainResult<()>;

    async fn get_user_by_id(&self, user_id: &str) -> UserDomainResult<Option<User>>;

    async fn get_user_by_username_or_email(
        &self,
        username: &str,
        email: &str,
    ) -> UserDomainResult<Option<User>>;

    async fn user_exists(
        &self,
        username: &str,
        email: &str,
        email_status: Option<EmailStatus>,
    ) -> UserDomainResult<bool>;

    async fn upsert_user<'a>(
        &self,
        user: User,
        session: Option<&'a mut ClientSession>,
    ) -> UserDomainResult<()>;

    async fn run_in_transaction(&self, f: Ftx) -> UserDomainResult<()>;
}
