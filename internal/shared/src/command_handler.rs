use async_trait::async_trait;

use crate::{auth::AppContext, types::AppResult};

#[async_trait]
pub trait CommandHanlder<C>: Sync + Send {
    async fn handle(&self, ctx: &AppContext, cmd: C) -> AppResult<()>;
}
