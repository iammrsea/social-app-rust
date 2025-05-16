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

pub struct RevokeBadge {
    pub user_id: String,
    pub badge: String,
}

pub struct RevokeBadgeHandler {
    repo: Arc<dyn UserRepository>,
    guard: Arc<dyn UserGuards>,
}

impl RevokeBadgeHandler {
    pub fn new(repo: Arc<dyn UserRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { repo, guard }
    }
}
#[async_trait]
impl CommandHanlder<RevokeBadge> for RevokeBadgeHandler {
    async fn handle(&self, ctx: &AppContext, cmd: RevokeBadge) -> AppResult<()> {
        let auth_user = get_auth_user_from_ctx(&ctx);
        self.guard
            .authorize(&auth_user.role, &UserPermission::RevokeBadge)?;
        self.repo
            .revoke_badge(
                &cmd.user_id,
                Box::new(|user| {
                    user.revoke_badge(cmd.badge);
                }),
            )
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::app::command::revoke_badge::{RevokeBadge, RevokeBadgeHandler};
    use crate::domain::{user::User, user_repository::MockUserRepository};
    use crate::guards::MockUserGuards;
    use mockall::predicate::eq;
    use shared::command_handler::CommandHanlder;
    use shared::guards::permissions::UserPermission;
    use shared::{
        auth::{AppContext, AuthUser},
        guards::roles::UserRole,
    };

    #[tokio::test]
    async fn revoke_badge_success() {
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_guard = MockUserGuards::new();

        let badge = "Helpful".to_string();
        let old_badge = badge.clone();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Admin), eq(UserPermission::RevokeBadge))
            .returning(|_, _| Ok(()));

        mock_user_repo
            .expect_revoke_badge()
            .withf(move |uuid, _| uuid == &User::test_user_id())
            .returning(move |_uid, update_fn| {
                let mut user = User::new_test_user(None);
                user.award_badge(old_badge.clone());
                update_fn(&mut user);
                assert_eq!(
                    0,
                    user.badges().len(),
                    "expected number of badges: {}, got: {}",
                    0,
                    user.badges().len()
                );
                Ok(())
            });
        let handler = RevokeBadgeHandler::new(Arc::new(mock_user_repo), Arc::new(mock_guard));
        let cmd = RevokeBadge {
            user_id: User::test_user_id(),
            badge,
        };

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn revoke_badge_unauthorized() {
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_guard = MockUserGuards::new();
        let badge = "Helpful".to_string();
        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Regular), eq(UserPermission::RevokeBadge))
            .returning(|_, _| Err(shared::errors::user::UserDomainError::Unauthorized.into()));

        mock_user_repo.expect_award_badge().never();

        let handler = RevokeBadgeHandler::new(Arc::new(mock_user_repo), Arc::new(mock_guard));
        let cmd = RevokeBadge {
            user_id: User::test_user_id(),
            badge,
        };
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }
}
