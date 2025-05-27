use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::{AppContext, get_auth_user_from_ctx},
    command_handler::CommandHanlder,
    guards::permissions::UserPermission,
};

use crate::domain::errors::UserDomainError;
use crate::domain::result::UserDomainResult;
use crate::guards::UserGuards;
use crate::infra::repository::user_repository::UserRepository;

pub struct MakeModerator {
    pub user_id: String,
}

pub struct MakeModeratorHandler {
    user_repo: Arc<UserRepository>,
    guard: Arc<dyn UserGuards>,
}

impl MakeModeratorHandler {
    pub fn new(user_repo: Arc<UserRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { user_repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<MakeModerator, UserDomainError> for MakeModeratorHandler {
    async fn handle(&self, ctx: &AppContext, cmd: MakeModerator) -> UserDomainResult<()> {
        let auth_user = get_auth_user_from_ctx(&ctx);
        self.guard
            .authorize(&auth_user.0.role, &UserPermission::MakeModerator)?;
        self.user_repo
            .make_moderator(&cmd.user_id, |user| {
                user.make_moderator();
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
    async fn make_moderator_success() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_guard = MockUserGuards::new();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Admin), eq(UserPermission::MakeModerator))
            .returning(|_, _| Ok(()));

        mock_user_repo
            .expect_make_moderator()
            .withf(move |uuid, _| uuid == User::test_user_id())
            .returning(move |_uid, update_fn| {
                let mut user = User::new_test_user(None);
                update_fn(&mut user);
                assert_eq!(
                    &UserRole::Moderator,
                    user.role(),
                    "expected user role:{:#?}, got: {:#?}",
                    UserRole::Moderator,
                    user.role()
                );
                Ok(())
            });
        let handler = MakeModeratorHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(mock_guard),
        );

        let cmd = MakeModerator {
            user_id: User::test_user_id(),
        };

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn make_moderator_unauthorized() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_guard = MockUserGuards::new();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Regular), eq(UserPermission::MakeModerator))
            .returning(|_, _| Err(UserDomainError::Unauthorized));

        mock_user_repo.expect_ban_user().never();

        let handler = MakeModeratorHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(mock_guard),
        );
        let cmd = MakeModerator {
            user_id: User::test_user_id(),
        };
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }
}
