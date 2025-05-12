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

pub struct GetUserById {
    pub id: String,
}
pub struct GetUserByIdHander {
    repo: Arc<dyn UserReadModelRepository>,
    guard: Arc<dyn UserGuards>,
}

impl GetUserByIdHander {
    pub fn new(repo: Arc<dyn UserReadModelRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl QueryHandler<GetUserById, UserReadModel> for GetUserByIdHander {
    async fn handle(&self, ctx: AppContext, cmd: GetUserById) -> AppResult<UserReadModel> {
        let auth_user = get_auth_user_from_ctx(ctx);
        self.guard
            .authorize(&auth_user.role, &UserPermission::ViewUser)?;
        let user = self.repo.get_user_by_id(&cmd.id).await?;
        if let Some(found_user) = user {
            return Ok(found_user);
        }
        Err(UserDomainError::UserNotFound.into())
    }
}
