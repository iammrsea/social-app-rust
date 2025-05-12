use std::sync::Arc;

use async_trait::async_trait;
use shared::{
    auth::{AppContext, get_auth_user_from_ctx},
    errors::user::UserDomainError,
    guards::permissions::UserPermission,
    query_handler::QueryHandler,
    types::AppResult,
};

use crate::domain::{
    user_read_model::UserReadModel, user_read_model_repository::UserReadModelRepository,
};
use crate::guards::UserGuards;

pub struct GetUserByEmail {
    pub email: String,
}
pub struct GetUserByEmailHander {
    repo: Arc<dyn UserReadModelRepository>,
    guard: Arc<dyn UserGuards>,
}

impl GetUserByEmailHander {
    pub fn new(repo: Arc<dyn UserReadModelRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl QueryHandler<GetUserByEmail, UserReadModel> for GetUserByEmailHander {
    async fn handle(&self, ctx: AppContext, cmd: GetUserByEmail) -> AppResult<UserReadModel> {
        let auth_user = get_auth_user_from_ctx(ctx);
        self.guard
            .authorize(&auth_user.role, &UserPermission::ViewUser)?;
        let user = self.repo.get_user_by_email(&cmd.email).await?;
        if let Some(found_user) = user {
            return Ok(found_user);
        }
        Err(UserDomainError::UserNotFound.into())
    }
}
