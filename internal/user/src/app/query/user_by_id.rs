use std::sync::Arc;

use async_trait::async_trait;
use shared::{
    auth::AuthenticatedUser,
    errors::user::UserDomainError,
    guards::{
        Guards,
        rbac::{permissions::Permission, roles::UserRole},
    },
    query_handler::QueryHandler,
    types::AppResult,
};

use crate::domain::{
    user_read_model::UserReadModel, user_read_model_repository::UserReadModelRepository,
};

pub struct GetUserById {
    pub id: String,
}
pub struct GetUserByIdHander {
    repo: Arc<dyn UserReadModelRepository>,
    guard: Arc<dyn Guards>,
}

impl GetUserByIdHander {
    pub fn new(repo: Arc<dyn UserReadModelRepository>, guard: Arc<dyn Guards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl QueryHandler<GetUserById, UserReadModel> for GetUserByIdHander {
    async fn handle(&self, cmd: GetUserById) -> AppResult<UserReadModel> {
        let auth_user = AuthenticatedUser::new(UserRole::Admin); // TODO: Get auth user from context
        self.guard
            .authorize(&auth_user.role, &Permission::ViewUser)?;
        let user = self.repo.get_user_by_id(&cmd.id).await?;
        if let Some(found_user) = user {
            return Ok(found_user);
        }
        Err(UserDomainError::UserNotFound.into())
    }
}
