use crate::domain::{
    result::UserDomainResult,
    user_read_model::{GetUsersOptions, GetUsersResult, UserReadModel},
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait UserReadModelRepositoryTrait: Send + Sync {
    async fn get_users(&self, opts: &GetUsersOptions) -> UserDomainResult<GetUsersResult>;
    async fn get_user_by_id(&self, id: &str) -> UserDomainResult<Option<UserReadModel>>;
    async fn get_user_by_email(&self, email: &str) -> UserDomainResult<Option<UserReadModel>>;
}
