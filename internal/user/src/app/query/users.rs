use std::sync::Arc;

use async_trait::async_trait;
use shared::{
    auth::{AppContext, get_auth_user_from_ctx},
    guards::permissions::UserPermission,
    pagination::{PaginatedQueryResult, PaginationInfo},
    query_handler::QueryHandler,
    types::AppResult,
};

use crate::domain::{
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
impl QueryHandler<GetUsersOptions, Result> for GetUsersHandler {
    async fn handle(&self, ctx: AppContext, cmd: GetUsersOptions) -> AppResult<Result> {
        let auth_user = get_auth_user_from_ctx(ctx);
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
