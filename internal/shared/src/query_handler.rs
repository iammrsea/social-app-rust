use async_trait::async_trait;

use crate::types::AppResult;

#[async_trait]
pub trait QueryHandler<C, D> {
    async fn handle(&self, cmd: C) -> AppResult<D>;
}
