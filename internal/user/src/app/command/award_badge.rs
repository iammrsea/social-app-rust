use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::AuthenticatedUser,
    command_handler::CommandHanlder,
    guards::{permissions::UserPermission, roles::UserRole},
    types::{AppResult, non_empty_string::NonEmptyString},
};

use crate::domain::user_repository::UserRepository;
use crate::guards::UserGuards;

pub struct RevokeBadge {
    pub user_id: String,
    pub badge: NonEmptyString,
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
    async fn handle(&self, cmd: RevokeBadge) -> AppResult<()> {
        let auth_user = AuthenticatedUser::new(UserRole::Admin); // TODO: Get auth user from context
        self.guard
            .authorize(&auth_user.role, &UserPermission::AwardBadge)?;
        self.repo
            .award_badge(
                &cmd.user_id,
                Box::new(|user| {
                    user.award_badge(cmd.badge);
                }),
            )
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::app::command::award_badge::{RevokeBadge, RevokeBadgeHandler};
    use crate::domain::{user::User, user_repository::MockUserRepository};
    use crate::guards::MockUserGuards;
    use mockall::predicate::eq;
    use shared::command_handler::CommandHanlder;
    use shared::{
        guards::{permissions::UserPermission::AwardBadge, roles::UserRole::Admin},
        types::non_empty_string::NonEmptyString,
    };

    #[tokio::test]
    async fn award_badge_success() {
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_guard = MockUserGuards::new();

        let badge = NonEmptyString::new("Helpful".into()).unwrap();

        mock_guard
            .expect_authorize()
            .with(eq(&Admin), eq(&AwardBadge))
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
        let handler = RevokeBadgeHandler::new(Arc::new(mock_user_repo), Arc::new(mock_guard));
        let cmd = RevokeBadge {
            user_id: User::test_user_id(),
            badge,
        };
        let result = handler.handle(cmd).await;
        assert!(result.is_ok())
    }
}
