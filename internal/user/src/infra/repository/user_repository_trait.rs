use crate::domain::result::UserDomainResult;
use crate::domain::user::{EmailStatus, User};

use super::db_transactions::{DBTransaction, RepoDB};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    // type Transaction;
    fn get_repo_db(&self) -> RepoDB;
    async fn create_account(&self, user: User) -> UserDomainResult<()>;

    async fn make_moderator<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()>;

    async fn change_username<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()>;

    async fn award_badge<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()>;

    async fn revoke_badge<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()>;

    async fn ban_user<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()>;

    async fn unban_user<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()>;

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
        session: Option<DBTransaction<'a>>,
    ) -> UserDomainResult<()>;
}
