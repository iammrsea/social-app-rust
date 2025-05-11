use infra::guards_impl::GuardsImpl;
use std::sync::Arc;

use user::app::user_service::UserService;

use infra::storage::{AppStorage, StorageEngine};

pub struct Services {
    pub user_service: UserService,
}
pub struct AppService {
    pub services: Services,
}

impl AppService {
    pub async fn build(engine: StorageEngine) -> Self {
        let storage = AppStorage::build(engine).await;
        let repos = storage.repos();
        let guard = Arc::new(GuardsImpl::new());
        let services = Services {
            user_service: UserService::new(repos.user_repo, repos.user_read_repo, guard),
            // Add more services for other app domains here
        };
        Self { services }
    }
}
