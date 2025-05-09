use std::sync::Arc;

use async_trait::async_trait;
use shared::{
    auth::AuthenticatedUser,
    guards::{
        Guards,
        rbac::{permissions::Permission, roles::UserRole},
    },
    pagination::{PaginatedQueryResult, PaginationInfo},
    query_handler::QueryHandler,
    types::AppResult,
};

use crate::domain::{
    user_read_model::UserReadModel,
    user_read_model_repository::{GetUsersOptions, UserReadModelRepository},
};

type Result = PaginatedQueryResult<UserReadModel>;

pub struct GetUsersHandler {
    repo: Arc<dyn UserReadModelRepository>,
    guard: Arc<dyn Guards>,
}

impl GetUsersHandler {
    pub fn new(repo: Arc<dyn UserReadModelRepository>, guard: Arc<dyn Guards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl QueryHandler<GetUsersOptions, Result> for GetUsersHandler {
    async fn handle(&self, cmd: GetUsersOptions) -> AppResult<Result> {
        let auth_user = AuthenticatedUser::new(UserRole::Admin); // TODO: Get auth user from context
        self.guard
            .authorize(&auth_user.role, &Permission::ListUsers)?;
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
