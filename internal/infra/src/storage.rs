use std::sync::Arc;

use user::infra::repository::{
    otp_repository::OtpRepository, user_read_model_repository::UserReadModelRepository,
    user_repository::UserRepository,
};

use mongo_storage::MongoDBStorage;

mod mongo_storage;

pub struct Repos {
    pub user_repo: Arc<UserRepository>,
    pub user_read_repo: Arc<UserReadModelRepository>,
    pub otp_repo: Arc<OtpRepository>,
}

pub enum StorageEngine {
    MongoDB,
    Memory,
    PostgreSQL,
    SQLite,
}
pub enum StorageSource {
    Mongo(MongoDBStorage),
    //Add other storage sources here
}

impl StorageSource {
    pub fn repos(&self) -> Repos {
        match self {
            StorageSource::Mongo(mongo) => mongo.repos(),
        }
    }
}

pub struct AppStorage;
impl AppStorage {
    pub async fn build(engine: StorageEngine) -> StorageSource {
        match engine {
            StorageEngine::MongoDB => StorageSource::Mongo(MongoDBStorage::new().await),
            _ => unimplemented!(),
        }
    }
}
