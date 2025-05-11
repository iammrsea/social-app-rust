use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::AuthenticatedUser,
    command_handler::CommandHanlder,
    errors::user::UserDomainError,
    guards::roles::UserRole,
    types::{AppResult, non_empty_string::NonEmptyString},
};

use crate::domain::user_repository::UserRepository;
use crate::guards::UserGuards;

pub struct ChangeUsername {
    pub user_id: String,
    pub username: NonEmptyString,
}

pub struct ChangeUsernameHandler {
    repo: Arc<dyn UserRepository>,
    guard: Arc<dyn UserGuards>,
}

impl ChangeUsernameHandler {
    pub fn new(repo: Arc<dyn UserRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<ChangeUsername> for ChangeUsernameHandler {
    async fn handle(&self, cmd: ChangeUsername) -> AppResult<()> {
        let auth_user = AuthenticatedUser::new(UserRole::Admin); // TODO: Get auth user from context

        self.guard.can_change_username(&auth_user.id, &auth_user)?;

        let exists = self.repo.user_exists(&cmd.username, "").await?;

        if exists {
            return Err(UserDomainError::UsernameTaken.into());
        }
        self.repo
            .change_username(
                &cmd.user_id,
                Box::new(|user| {
                    user.change_username(cmd.username);
                }),
            )
            .await?;
        Ok(())
    }
}
