use std::sync::Arc;

use async_trait::async_trait;
use user::domain::{
    user_read_model_repository::UserReadModelRepository, user_repository::UserRepository,
};

enum StorageEngine {
    MongoDB,
    Memory,
    PostgreSQL,
    SQLite,
}

struct AppStorage {}

impl AppStorage {
    fn build(engine: StorageEngine) -> impl StorageSources {
        match engine {
            StorageEngine::MongoDB => MongoDBStorage::new(),
            _ => unimplemented!(),
        }
    }
}

struct Repos {
    user_repo: Arc<dyn UserRepository>,
    user_read_repo: Arc<dyn UserReadModelRepository>,
}

#[async_trait]
pub trait StorageSources: Send + Sync {
    fn repos(&self) -> ();
    // fn repos(&self) -> Repos;
    async fn close_storage(&self);
}

pub struct MongoDBStorage;

impl MongoDBStorage {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl StorageSources for MongoDBStorage {
    fn repos(&self) -> () {}
    async fn close_storage(&self) {}
}
