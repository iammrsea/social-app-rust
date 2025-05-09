use async_trait::async_trait;

use crate::types::AppResult;

#[async_trait]
pub trait CommandHanlder<C>: Sync + Send {
    async fn handle(&self, cmd: C) -> AppResult<()>;
}
