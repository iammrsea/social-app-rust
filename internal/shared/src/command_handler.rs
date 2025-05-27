use async_trait::async_trait;

use crate::auth::AppContext;

#[async_trait]
pub trait CommandHanlder<C, E, R = ()>: Sync + Send {
    async fn handle(&self, ctx: &AppContext, cmd: C) -> Result<R, E>;
}
