use std::sync::Arc;

use async_trait::async_trait;
use shared::{
    auth::{AppContext, get_auth_user_from_ctx},
    guards::permissions::UserPermission,
    query_handler::QueryHandler,
};

use crate::domain::{
    errors::UserDomainError, result::UserDomainResult, user_read_model::UserReadModel,
};
use crate::guards::UserGuards;
use crate::infra::repository::user_read_model_repository::UserReadModelRepository;

pub struct GetUserByEmail {
    pub email: String,
}
pub struct GetUserByEmailHander {
    user_repo: Arc<UserReadModelRepository>,
    guard: Arc<dyn UserGuards>,
}

impl GetUserByEmailHander {
    pub fn new(user_repo: Arc<UserReadModelRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { user_repo, guard }
    }
}

#[async_trait]
impl QueryHandler<GetUserByEmail, UserReadModel, UserDomainError> for GetUserByEmailHander {
    async fn handle(
        &self,
        ctx: &AppContext,
        cmd: GetUserByEmail,
    ) -> UserDomainResult<UserReadModel> {
        let auth_user = get_auth_user_from_ctx(&ctx);
        self.guard
            .authorize(&auth_user.0.role, &UserPermission::ViewUser)?;
        let user = self.user_repo.get_user_by_email(&cmd.email).await?;
        if let Some(found_user) = user {
            return Ok(found_user);
        }
        Err(UserDomainError::UserNotFound.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use shared::{auth::AuthUser, guards::roles::UserRole};
    use std::sync::Arc;

    use crate::{
        guards::MockUserGuards,
        infra::repository::user_read_model_repository_trait::MockUserReadModelRepositoryTrait,
    };

    #[tokio::test]
    async fn get_user_by_email_success() {
        let mut mock_user_read_repo = MockUserReadModelRepositoryTrait::new();
        let mut mock_guard = MockUserGuards::new();

        let cmd = GetUserByEmail {
            email: "test@gmail.com".into(),
        };

        let user_email = cmd.email.clone();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Regular), eq(UserPermission::ViewUser))
            .returning(|_, _| Ok(()));

        mock_user_read_repo
            .expect_get_user_by_email()
            .withf(move |email| email == user_email)
            .returning(|_| {
                let user = UserReadModel::new_test_user_read_model();
                Ok(Some(user))
            });

        let handler = GetUserByEmailHander::new(
            Arc::new(UserReadModelRepository::Mock(mock_user_read_repo)),
            Arc::new(mock_guard),
        );
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn get_user_by_email_not_found() {
        let mut mock_user_read_repo = MockUserReadModelRepositoryTrait::new();
        let mut mock_guard = MockUserGuards::new();

        let cmd = GetUserByEmail {
            email: "test@gmail.com".into(),
        };

        let user_email = cmd.email.clone();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Regular), eq(UserPermission::ViewUser))
            .returning(|_, _| Ok(()));

        mock_user_read_repo
            .expect_get_user_by_email()
            .withf(move |email| email == user_email)
            .returning(|_| Ok(None));

        let handler = GetUserByEmailHander::new(
            Arc::new(UserReadModelRepository::Mock(mock_user_read_repo)),
            Arc::new(mock_guard),
        );
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err())
    }
}
