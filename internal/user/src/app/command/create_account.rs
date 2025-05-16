use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::{AppContext, get_auth_user_from_ctx},
    command_handler::CommandHanlder,
    errors::user::UserDomainError,
    guards::{permissions::UserPermission, roles::UserRole},
    types::AppResult,
};

use crate::domain::{user::User, user_repository::UserRepository};
use crate::guards::UserGuards;

pub struct CreateAccount {
    pub email: String,
    pub username: String,
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
    async fn handle(&self, ctx: &AppContext, cmd: CreateAccount) -> AppResult<()> {
        let auth_user = get_auth_user_from_ctx(&ctx);
        self.guard
            .authorize(&auth_user.role, &UserPermission::CreateAccount)?;

        let exists = self.repo.user_exists(&cmd.username, &cmd.email).await?;
        if exists {
            return Err(UserDomainError::UsernameOrEmailTaken.into());
        }
        let user = User::new(cmd.email, cmd.username, UserRole::Regular);
        self.repo.create_account(user).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use shared::{
        auth::{AppContext, AuthUser},
        command_handler::CommandHanlder,
        guards::{permissions::UserPermission, roles::UserRole},
    };

    use crate::{
        app::command::create_account::{CreateAccount, CreateAccountHandler},
        domain::user_repository::MockUserRepository,
        guards::MockUserGuards,
    };

    #[tokio::test]
    async fn create_account_success() {
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_guard = MockUserGuards::new();

        let cmd = CreateAccount {
            email: "test@gmail.com".into(),
            username: "test".into(),
        };

        let expected_username = cmd.username.clone();
        let expected_email = cmd.email.clone();

        let auth_user = AuthUser::new_test_auth_user(UserRole::Guest);

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Guest), eq(UserPermission::CreateAccount))
            .returning(|_, _| Ok(()));

        mock_user_repo
            .expect_user_exists()
            .with(
                eq(expected_username.to_string()),
                eq(expected_email.to_string()),
            )
            .returning(|_, _| Ok(false));

        mock_user_repo
            .expect_create_account()
            .returning(move |user| {
                assert_eq!(expected_email.to_string(), user.email());
                assert_eq!(expected_username.to_string(), user.username());
                Ok(())
            });

        let handler = CreateAccountHandler::new(Arc::new(mock_user_repo), Arc::new(mock_guard));
        let ctx = AppContext::new().with_user(auth_user);
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn create_account_failed_email_or_username_exists() {
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_guard = MockUserGuards::new();

        let cmd = CreateAccount {
            email: "test@gmail.com".into(),
            username: "test".into(),
        };

        let expected_username = cmd.username.clone();
        let expected_email = cmd.email.clone();

        let auth_user = AuthUser::new_test_auth_user(UserRole::Guest);

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Guest), eq(UserPermission::CreateAccount))
            .returning(|_, _| Ok(()));

        mock_user_repo
            .expect_user_exists()
            .with(
                eq(expected_username.to_string()),
                eq(expected_email.to_string()),
            )
            .returning(|_, _| Ok(true));

        mock_user_repo.expect_create_account().never();

        let handler = CreateAccountHandler::new(Arc::new(mock_user_repo), Arc::new(mock_guard));
        let ctx = AppContext::new().with_user(auth_user);
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err())
    }
}
