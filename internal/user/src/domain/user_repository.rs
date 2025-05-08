use async_trait::async_trait;

use shared::types::AppResult;

use super::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_account(&self, user: User) -> AppResult<()>;

    async fn make_moderator<F>(&self, user_id: &str, update_fn: F) -> AppResult<()>
    where
        F: FnOnce(&mut User) -> AppResult<()> + Send;

    async fn change_username<F>(&self, user_id: &str, update_fn: F) -> AppResult<()>
    where
        F: FnOnce(&mut User) -> AppResult<()> + Send;

    async fn award_badge<F>(&self, user_id: &str, update_fn: F) -> AppResult<()>
    where
        F: FnOnce(&mut User) -> AppResult<()> + Send;

    async fn ban_user<F>(&self, user_id: &str, update_fn: F) -> AppResult<()>
    where
        F: FnOnce(&mut User) -> AppResult<()> + Send;

    async fn unban_user<F>(&self, user_id: &str, update_fn: F) -> AppResult<()>
    where
        F: FnOnce(&mut User) -> AppResult<()> + Send;

    async fn get_user_by_id(&self, user_id: &str) -> AppResult<Option<User>>;

    async fn user_exists(&self, user_id: &str, email: &str) -> AppResult<bool>;
}
