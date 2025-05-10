use std::sync::Arc;

use user::domain::{
    user_read_model_repository::UserReadModelRepository, user_repository::UserRepository,
};

use mongo_storage::MongoDBStorage;

mod mongo_storage;

pub struct Repos {
    pub user_repo: Arc<dyn UserRepository>,
    pub user_read_repo: Arc<dyn UserReadModelRepository>,
}

pub enum StorageEngine {
    MongoDB,
    Memory,
    PostgreSQL,
    SQLite,
}

pub struct AppStorage;

impl AppStorage {
    pub async fn build(engine: StorageEngine) -> Box<dyn StorageSources> {
        match engine {
            StorageEngine::MongoDB => MongoDBStorage::new().await,
            _ => unimplemented!(),
        }
    }
}

pub trait StorageSources: Send + Sync {
    fn repos(&self) -> Repos;
}
