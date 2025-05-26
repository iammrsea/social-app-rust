use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::{AppContext, get_auth_user_from_ctx},
    command_handler::CommandHanlder,
};

use crate::domain::{errors::UserDomainError, result::UserDomainResult, user::EmailStatus};
use crate::guards::UserGuards;
use crate::infra::repository::user_repository::UserRepository;

#[derive(Clone)]
pub struct ChangeUsername {
    pub user_id: String,
    pub username: String,
}

pub struct ChangeUsernameHandler {
    user_repo: Arc<UserRepository>,
    guard: Arc<dyn UserGuards>,
}

impl ChangeUsernameHandler {
    pub fn new(user_repo: Arc<UserRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { user_repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<ChangeUsername, UserDomainError> for ChangeUsernameHandler {
    async fn handle(&self, ctx: &AppContext, cmd: ChangeUsername) -> UserDomainResult<()> {
        let auth_user = get_auth_user_from_ctx(&ctx);

        self.guard.can_change_username(&auth_user.id, &auth_user)?;

        let exists = self
            .user_repo
            .user_exists(&cmd.username, "", Some(EmailStatus::Verified))
            .await?;

        if exists {
            return Err(UserDomainError::UsernameTaken.into());
        }
        self.user_repo
            .change_username(&cmd.user_id, |user| {
                user.change_username(cmd.username);
            })
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::user::User;
    use crate::guards::MockUserGuards;
    use crate::infra::repository::user_repository_trait::MockUserRepositoryTrait;
    use mockall::predicate::eq;
    use shared::{
        auth::{AppContext, AuthUser},
        guards::roles::UserRole,
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn change_username_success() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_guard = MockUserGuards::new();

        let auth_user = AuthUser::new_test_auth_user(UserRole::Regular);

        let cmd = ChangeUsername {
            user_id: auth_user.id.clone(),
            username: "newUsername".into(),
        };
        let expected_auth_user_id = auth_user.id.clone();
        let expected_auth_user = auth_user.clone();
        let expected_username = cmd.username.clone();

        mock_guard
            .expect_can_change_username()
            .with(eq(expected_auth_user_id.clone()), eq(expected_auth_user))
            .returning(|_, _| Ok(()));

        mock_user_repo
            .expect_user_exists()
            .with(
                eq(expected_username.clone().to_string()),
                eq(""),
                eq(Some(EmailStatus::Verified)),
            )
            .returning(|_, _, _| Ok(false));

        mock_user_repo
            .expect_change_username()
            .withf(move |uuid, _| uuid == expected_auth_user_id.clone())
            .returning(move |_uid, update_fn| {
                let mut user = User::new_test_user(None);
                update_fn(&mut user);
                assert_eq!(
                    &expected_username.to_string(),
                    user.username(),
                    "expected username: {}, got: {}",
                    &expected_username.to_string(),
                    user.username()
                );
                Ok(())
            });
        let handler = ChangeUsernameHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(mock_guard),
        );

        let ctx = AppContext::new().with_user(auth_user);

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn change_username_failed_username_taken() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_guard = MockUserGuards::new();

        let auth_user = AuthUser::new_test_auth_user(UserRole::Regular);

        let cmd = ChangeUsername {
            user_id: auth_user.id.clone(),
            username: "newUsername".into(),
        };
        let expected_auth_user_id = auth_user.id.clone();
        let expected_auth_user = auth_user.clone();
        let expected_username = cmd.username.clone();

        mock_guard
            .expect_can_change_username()
            .with(eq(expected_auth_user_id), eq(expected_auth_user))
            .returning(|_, _| Ok(()));

        mock_user_repo
            .expect_user_exists()
            .with(
                eq(expected_username.to_string()),
                eq(""),
                eq(Some(EmailStatus::Verified)),
            )
            .returning(|_, _, _| Ok(true));

        mock_user_repo.expect_award_badge().never();

        let handler = ChangeUsernameHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(mock_guard),
        );

        let ctx = AppContext::new().with_user(auth_user);
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }
}
