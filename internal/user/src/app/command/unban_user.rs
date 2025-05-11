use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::AuthenticatedUser,
    command_handler::CommandHanlder,
    guards::{permissions::UserPermission, roles::UserRole},
    types::AppResult,
};

use crate::domain::user_repository::UserRepository;
use crate::guards::UserGuards;

pub struct UnbanUser {
    pub user_id: String,
}

pub struct UnbanUserHandler {
    repo: Arc<dyn UserRepository>,
    guard: Arc<dyn UserGuards>,
}

impl UnbanUserHandler {
    pub fn new(repo: Arc<dyn UserRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<UnbanUser> for UnbanUserHandler {
    async fn handle(&self, cmd: UnbanUser) -> AppResult<()> {
        let auth_user = AuthenticatedUser::new(UserRole::Admin); // TODO: Get auth user from context
        self.guard
            .authorize(&auth_user.role, &UserPermission::UnbanUser)?;
        self.repo
            .unban_user(
                &cmd.user_id,
                Box::new(|user| {
                    user.unban();
                }),
            )
            .await?;
        Ok(())
    }
}
