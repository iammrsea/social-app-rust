use async_trait::async_trait;

use crate::auth::AppContext;

#[async_trait]
pub trait QueryHandler<C, D, E> {
    async fn handle(&self, ctx: &AppContext, cmd: C) -> Result<D, E>;
}
