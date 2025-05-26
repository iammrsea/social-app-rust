use mongodb::{Client, bson::doc, options::ClientOptions};
use std::time::Duration;
use tokio::time::timeout;
use tracing::info;

use crate::config::{Config, MongoDbConfig};
use shared::types::AppResult;
use std::sync::Arc;

use auth::mongoimpl::MongoOtpRepository;
use auth::repository::OtpRepository;
use user::infra::mongoimpl::{
    user_read_model_repository::MongoUserReadModelRepository, user_repository::MongoUserRepository,
};
use user::infra::repository::{
    user_read_model_repository::UserReadModelRepository, user_repository::UserRepository,
};

use super::*;

pub struct MongoDBStorage {
    client: Client,
    cfg: MongoDbConfig,
}

impl MongoDBStorage {
    pub async fn new() -> Self {
        let cfg = Config::build();
        let mongo_cfg = cfg.build_mongodb_config();
        info!("Connecting to MongoDB...");
        let client = connect(&mongo_cfg)
            .await
            .expect("Unable to contect to MongoDB");

        info!("✅ Connected to MongoDB");
        Self {
            client,
            cfg: mongo_cfg,
        }
    }
    pub fn repos(&self) -> Repos {
        let db = self.client.database(&self.cfg.database_name);
        Repos {
            user_repo: Arc::new(UserRepository::MongoDb(MongoUserRepository::new(
                db.clone(),
            ))),
            user_read_repo: Arc::new(UserReadModelRepository::MongoDb(
                MongoUserReadModelRepository::new(db.clone()),
            )),
            otp_repo: OtpRepository::MongoDb(MongoOtpRepository::new(db.clone())),
        }
    }
}

async fn connect(cf: &MongoDbConfig) -> AppResult<Client> {
    let mut options = ClientOptions::parse(&cf.uri).await?;
    options.max_pool_size = Some(cf.max_pool_size);
    options.min_pool_size = Some(cf.min_pool_size);
    options.max_idle_time = Some(Duration::from_secs(cf.conn_idle_time_secs));
    options.retry_reads = Some(cf.retry_reads);
    options.retry_writes = Some(cf.retry_writes);
    if let Some(rs) = &cf.replica_set {
        options.repl_set_name = Some(rs.clone());
    }

    let client = Client::with_options(options)?;
    let ping_result = timeout(
        Duration::from_secs(cf.timeout_secs),
        client.database("admin").run_command(doc! { "ping": 1 }),
    )
    .await;

    info!("Ping response: {:?}", ping_result);
    Ok(client)
}
