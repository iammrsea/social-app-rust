use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::AuthenticatedUser,
    command_handler::CommandHanlder,
    errors::user::UserDomainError,
    guards::{permissions::UserPermission, roles::UserRole},
    types::{AppResult, non_empty_string::NonEmptyString},
};

use crate::domain::{user::User, user_repository::UserRepository};
use crate::guards::UserGuards;

pub struct CreateAccount {
    pub email: NonEmptyString,
    pub username: NonEmptyString,
}

pub struct CreateAccountHandler {
    repo: Arc<dyn UserRepository>,
    guard: Arc<dyn UserGuards>,
}

impl CreateAccountHandler {
    pub fn new(repo: Arc<dyn UserRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<CreateAccount> for CreateAccountHandler {
    async fn handle(&self, cmd: CreateAccount) -> AppResult<()> {
        let auth_user = AuthenticatedUser::new(UserRole::Admin); // TODO: Get auth user from context
        self.guard
            .authorize(&auth_user.role, &UserPermission::CreateAccount)?;

        let exists = self.repo.user_exists(&cmd.username, &cmd.username).await?;
        if exists {
            return Err(UserDomainError::UsernameOrEmailTaken.into());
        }
        let user = User::new(cmd.email, cmd.username, UserRole::Regular);
        self.repo.create_account(user).await?;
        Ok(())
    }
}
