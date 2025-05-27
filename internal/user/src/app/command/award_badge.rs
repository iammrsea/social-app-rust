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

pub struct AwardBadge {
    pub user_id: String,
    pub badge: String,
}

pub struct AwardBadgeHandler {
    user_repo: Arc<UserRepository>,
    guard: Arc<dyn UserGuards>,
}

impl AwardBadgeHandler {
    pub fn new(user_repo: Arc<UserRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { user_repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<AwardBadge, UserDomainError> for AwardBadgeHandler {
    async fn handle(&self, ctx: &AppContext, cmd: AwardBadge) -> UserDomainResult<()> {
        let auth_user = get_auth_user_from_ctx(&ctx);
        self.guard
            .authorize(&auth_user.0.role, &UserPermission::AwardBadge)?;
        self.user_repo
            .award_badge(&cmd.user_id, |user| {
                user.award_badge(cmd.badge);
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
    async fn award_badge_success() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_guard = MockUserGuards::new();

        let badge = "Helpful".to_string();
        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Admin), eq(UserPermission::AwardBadge))
            .returning(|_, _| Ok(()));

        mock_user_repo
            .expect_award_badge()
            .withf(move |uuid, _| uuid == &User::test_user_id())
            .returning(|_uid, update_fn| {
                let mut user = User::new_test_user(None);
                update_fn(&mut user);
                assert_eq!(
                    1,
                    user.badges().len(),
                    "expected number of badges: {}, got: {}",
                    1,
                    user.badges().len()
                );
                Ok(())
            });
        let handler = AwardBadgeHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(mock_guard),
        );
        let cmd = AwardBadge {
            user_id: User::test_user_id(),
            badge,
        };

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn award_badge_unauthorized() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_guard = MockUserGuards::new();
        let badge = "Helpful".to_string();
        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Regular), eq(UserPermission::AwardBadge))
            .returning(|_, _| Err(UserDomainError::Unauthorized));

        mock_user_repo.expect_award_badge().never();

        let handler = AwardBadgeHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(mock_guard),
        );
        let cmd = AwardBadge {
            user_id: User::test_user_id(),
            badge,
        };
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }
}
