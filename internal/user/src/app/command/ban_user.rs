use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::{AppContext, get_auth_user_from_ctx},
    command_handler::CommandHanlder,
    guards::permissions::UserPermission,
    types::{AppResult, non_empty_string::NonEmptyString},
};

use crate::domain::{user::BanType, user_repository::UserRepository};
use crate::guards::UserGuards;

pub struct BanUser {
    pub user_id: String,
    pub reason: NonEmptyString,
    pub ban_type: BanType,
}

pub struct BanUserHandler {
    repo: Arc<dyn UserRepository>,
    guard: Arc<dyn UserGuards>,
}

impl BanUserHandler {
    pub fn new(repo: Arc<dyn UserRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<BanUser> for BanUserHandler {
    async fn handle(&self, ctx: &AppContext, cmd: BanUser) -> AppResult<()> {
        let auth_user = get_auth_user_from_ctx(&ctx);
        self.guard
            .authorize(&auth_user.role, &UserPermission::BanUser)?;
        self.repo
            .ban_user(
                &cmd.user_id,
                Box::new(|user| {
                    user.ban(cmd.reason, cmd.ban_type);
                }),
            )
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::app::command::ban_user::{BanUser, BanUserHandler};
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
    async fn ban_user_success() {
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_guard = MockUserGuards::new();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Admin), eq(UserPermission::BanUser))
            .returning(|_, _| Ok(()));

        mock_user_repo
            .expect_ban_user()
            .withf(move |uuid, _| uuid == &User::test_user_id())
            .returning(|_uid, update_fn| {
                let mut user = User::new_test_user(None);
                update_fn(&mut user);
                let ban_status = user.ban_status().unwrap();
                assert_eq!(true, ban_status.is_banned(), "expected user to be banned",);
                Ok(())
            });
        let handler = BanUserHandler::new(Arc::new(mock_user_repo), Arc::new(mock_guard));
        let cmd = BanUser {
            user_id: User::test_user_id(),
            reason: NonEmptyString::new("Abuse".into()).unwrap(),
            ban_type: BanType::Indefinite,
        };

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn ban_user_unauthorized() {
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_guard = MockUserGuards::new();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Regular), eq(UserPermission::BanUser))
            .returning(|_, _| Err(shared::errors::user::UserDomainError::Unauthorized.into()));

        mock_user_repo.expect_ban_user().never();

        let handler = BanUserHandler::new(Arc::new(mock_user_repo), Arc::new(mock_guard));
        let cmd = BanUser {
            user_id: User::test_user_id(),
            reason: NonEmptyString::new("Abuse".into()).unwrap(),
            ban_type: BanType::Indefinite,
        };
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }
}
