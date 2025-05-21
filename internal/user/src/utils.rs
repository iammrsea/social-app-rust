use std::sync::Arc;

use mongodb::ClientSession;

use crate::domain::{
    errors::UserDomainResult,
    user_repository::{Ftx, UserRepository},
};

pub async fn run_user_transaction<Fut>(
    repo: Arc<dyn UserRepository>,
    f: impl for<'a> FnOnce(&'a mut ClientSession) -> Fut + Send + 'static,
) -> UserDomainResult<()>
where
    Fut: Future<Output = UserDomainResult<()>> + Send + 'static,
{
    repo.run_in_transaction(Box::new(move |session| Box::pin(f(session))))
        .await
}
