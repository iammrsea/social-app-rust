use async_trait::async_trait;

use crate::{auth::AppContext, types::AppResult};

#[async_trait]
pub trait QueryHandler<C, D> {
    async fn handle(&self, ctx: &AppContext, cmd: C) -> AppResult<D>;
}
