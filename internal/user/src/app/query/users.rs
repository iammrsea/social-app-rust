use std::sync::Arc;

use async_trait::async_trait;
use shared::{
    auth::{AppContext, get_auth_user_from_ctx},
    guards::permissions::UserPermission,
    pagination::{PaginatedQueryResult, PaginationInfo},
    query_handler::QueryHandler,
};

use crate::domain::{
    errors::{UserDomainError, UserDomainResult},
    user_read_model::UserReadModel,
    user_read_model_repository::{GetUsersOptions, UserReadModelRepository},
};
use crate::guards::UserGuards;

type Result = PaginatedQueryResult<UserReadModel>;

pub struct GetUsersHandler {
    repo: Arc<dyn UserReadModelRepository>,
    guard: Arc<dyn UserGuards>,
}

impl GetUsersHandler {
    pub fn new(repo: Arc<dyn UserReadModelRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl QueryHandler<GetUsersOptions, Result, UserDomainError> for GetUsersHandler {
    async fn handle(&self, ctx: &AppContext, cmd: GetUsersOptions) -> UserDomainResult<Result> {
        let auth_user = get_auth_user_from_ctx(&ctx);
        self.guard
            .authorize(&auth_user.role, &UserPermission::ListUsers)?;
        let resp = self.repo.get_users(&cmd).await?;
        let result = Result {
            data: resp.users,
            pagination_info: PaginationInfo {
                has_next: resp.has_next,
            },
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use shared::{auth::AuthUser, guards::roles::UserRole};

    use crate::{
        domain::user_read_model_repository::{
            GetUsersResult, MockUserReadModelRepository, SortDirection,
        },
        guards::MockUserGuards,
    };

    #[tokio::test]
    async fn get_users_success() {
        let mut mock_user_read_repo = MockUserReadModelRepository::new();
        let mut mock_guard = MockUserGuards::new();

        let cmd = GetUsersOptions {
            after: Some("iie393939393".into()),
            first: 30,
            sort_direction: SortDirection::DESC,
        };

        let cmd_cpy = cmd.clone();

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Moderator), eq(UserPermission::ListUsers))
            .returning(|_, _| Ok(()));

        mock_user_read_repo
            .expect_get_users()
            .withf(move |c| c == &cmd_cpy)
            .returning(|_| {
                let users = vec![UserReadModel::new_test_user_read_model()];
                Ok(GetUsersResult {
                    has_next: false,
                    users,
                })
            });
        let handler = GetUsersHandler::new(Arc::new(mock_user_read_repo), Arc::new(mock_guard));
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Moderator));

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn get_users_unauthorized() {
        let mut mock_user_read_repo = MockUserReadModelRepository::new();
        let mut mock_guard = MockUserGuards::new();

        let cmd = GetUsersOptions {
            after: Some("iie393939393".into()),
            first: 30,
            sort_direction: SortDirection::DESC,
        };

        mock_guard
            .expect_authorize()
            .with(eq(UserRole::Regular), eq(UserPermission::ListUsers))
            .returning(|_, _| Err(UserDomainError::Unauthorized));

        mock_user_read_repo.expect_get_users().never();

        let handler = GetUsersHandler::new(Arc::new(mock_user_read_repo), Arc::new(mock_guard));
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err())
    }
}
