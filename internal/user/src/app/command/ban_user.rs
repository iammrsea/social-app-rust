use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::{AppContext, get_auth_user_from_ctx},
    command_handler::CommandHanlder,
    guards::permissions::UserPermission,
};

use crate::domain::{errors::UserDomainError, result::UserDomainResult, user::BanType};
use crate::infra::repository::user_repository::UserRepository;

use crate::guards::UserGuards;

pub struct BanUser {
    pub user_id: String,
    pub reason: String,
    pub ban_type: BanType,
}

pub struct BanUserHandler {
    user_repo: Arc<UserRepository>,
    guard: Arc<dyn UserGuards>,
}

impl BanUserHandler {
    pub fn new(user_repo: Arc<UserRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { user_repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<BanUser, UserDomainError> for BanUserHandler {
    async fn handle(&self, ctx: &AppContext, cmd: BanUser) -> UserDomainResult<()> {
        let auth_user = get_auth_user_from_ctx(&ctx);
        self.guard
            .authorize(&auth_user.0.role, &UserPermission::BanUser)?;
        self.user_repo
            .ban_user(&cmd.user_id, |user| {
                user.ban(cmd.reason, cmd.ban_type);
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
    async fn ban_user_success() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
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
        let handler = BanUserHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(mock_guard),
        );
        let cmd = BanUser {
            user_id: User::test_user_id(),
            reason: "Abuse".into(),
            ban_type: BanType::Indefinite,
        };

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn ban_user_unauthorized() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_guard = MockUserGuards::new();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Regular), eq(UserPermission::BanUser))
            .returning(|_, _| Err(UserDomainError::Unauthorized));

        mock_user_repo.expect_ban_user().never();

        let handler = BanUserHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(mock_guard),
        );
        let cmd = BanUser {
            user_id: User::test_user_id(),
            reason: "Abuse".into(),
            ban_type: BanType::Indefinite,
        };
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }
}
