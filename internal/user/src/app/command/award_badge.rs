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
            .authorize(&auth_user.role, &UserPermission::RevokeBadge)?;
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
    use crate::domain::{user::User, user_repository::MockUserRepository};
    use crate::guards::MockUserGuards;
    use mockall::predicate::eq;
    use shared::{
        guards::{permissions::UserPermission::AwardBadge, roles::UserRole::Admin},
        types::non_empty_string::NonEmptyString,
    };

    #[tokio::test]
    async fn award_badge_success() {
        let mut _mock_user_repo = MockUserRepository::new();
        let mut mock_guard = MockUserGuards::new();

        let test_user = User::new_test_user(None);

        let _user_id: String = test_user.id().into();
        let _badge = NonEmptyString::new("Helpful".into()).unwrap();

        mock_guard
            .expect_authorize()
            .with(eq(&Admin), eq(&AwardBadge))
            .returning(|_, _| Ok(()));
    }
}
