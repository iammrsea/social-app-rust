use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::{AppContext, get_auth_user_from_ctx},
    command_handler::CommandHanlder,
    guards::permissions::UserPermission,
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
    async fn handle(&self, ctx: AppContext, cmd: UnbanUser) -> AppResult<()> {
        let auth_user = get_auth_user_from_ctx(ctx);
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::app::command::unban_user::{UnbanUser, UnbanUserHandler};
    use crate::domain::user::BanType;
    use crate::domain::{user::User, user_repository::MockUserRepository};
    use crate::guards::MockUserGuards;
    use mockall::predicate::eq;
    use shared::command_handler::CommandHanlder;
    use shared::guards::permissions::UserPermission;
    use shared::{
        auth::{AppContext, AuthUser},
        guards::roles::UserRole,
        types::non_empty_string::NonEmptyString,
    };

    #[tokio::test]
    async fn unban_user_success() {
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_guard = MockUserGuards::new();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Admin), eq(UserPermission::UnbanUser))
            .returning(|_, _| Ok(()));

        mock_user_repo
            .expect_unban_user()
            .withf(move |uuid, _| uuid == &User::test_user_id())
            .returning(|_uid, update_fn| {
                let mut user = User::new_test_user(None);
                let reason = NonEmptyString::new("abuse".into()).unwrap();
                user.ban(reason, BanType::Indefinite);
                update_fn(&mut user);
                assert_eq!(None, user.ban_status(), "expected user not to be banned",);
                Ok(())
            });
        let handler = UnbanUserHandler::new(Arc::new(mock_user_repo), Arc::new(mock_guard));
        let cmd = UnbanUser {
            user_id: User::test_user_id(),
        };

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));

        let result = handler.handle(ctx, cmd).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn unban_user_unauthorized() {
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_guard = MockUserGuards::new();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Regular), eq(UserPermission::UnbanUser))
            .returning(|_, _| Err(shared::errors::user::UserDomainError::Unauthorized.into()));

        mock_user_repo.expect_unban_user().never();

        let handler = UnbanUserHandler::new(Arc::new(mock_user_repo), Arc::new(mock_guard));
        let cmd = UnbanUser {
            user_id: User::test_user_id(),
        };
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let result = handler.handle(ctx, cmd).await;
        assert!(result.is_err());
    }
}
