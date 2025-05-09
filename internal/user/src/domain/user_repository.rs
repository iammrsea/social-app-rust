use async_trait::async_trait;

use shared::types::AppResult;

use super::user::User;

pub type F = Box<dyn FnOnce(&mut User) -> AppResult<()> + Send>;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_account(&self, user: User) -> AppResult<()>;

    async fn make_moderator(&self, user_id: &str, update_fn: F) -> AppResult<()>;

    async fn change_username(&self, user_id: &str, update_fn: F) -> AppResult<()>;

    async fn award_badge(&self, user_id: &str, update_fn: F) -> AppResult<()>;

    async fn revoke_badge(&self, user_id: &str, update_fn: F) -> AppResult<()>;

    async fn ban_user(&self, user_id: &str, update_fn: F) -> AppResult<()>;

    async fn unban_user(&self, user_id: &str, update_fn: F) -> AppResult<()>;

    async fn get_user_by_id(&self, user_id: &str) -> AppResult<Option<User>>;

    async fn user_exists(&self, username: &str, email: &str) -> AppResult<bool>;
}
