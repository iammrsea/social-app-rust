use async_trait::async_trait;
use shared::types::AppResult;

use super::user_read_model::UserReadModel;

#[derive(Debug, Clone, PartialEq)]
pub struct GetUsersOptions {
    pub first: u32,
    pub after: String,
    pub sort_direction: SortDirection,
}
#[derive(Debug, Clone, PartialEq)]
pub enum SortDirection {
    ASC,
    DESC,
}

#[derive(Debug)]
pub struct GetUsersResult {
    pub users: Vec<UserReadModel>,
    pub has_next: bool,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserReadModelRepository: Send + Sync {
    async fn get_users(&self, opts: &GetUsersOptions) -> AppResult<GetUsersResult>;
    async fn get_user_by_id(&self, id: &str) -> AppResult<Option<UserReadModel>>;
    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<UserReadModel>>;
}
