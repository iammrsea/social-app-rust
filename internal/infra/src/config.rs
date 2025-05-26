use std::env::var;
use std::fmt::Debug;
use std::path::PathBuf;

use dotenvy::dotenv;
use once_cell::sync::OnceCell;

use utils::{get_env_bool, get_env_num};

const MONGODB_URI: &str = "MONGODB_URI";
const MONGODB_NAME: &str = "MONGODB_NAME";
const MONGODB_REPLICA_SET: &str = "MONGODB_REPLICA_SET";
const TIMEOUT: &str = "TIMEOUT";
const MAX_POOL_SIZE: &str = "MAX_POOL_SIZE";
const MIN_POOL_SIZE: &str = "MIN_POOL_SIZE";
const CONN_IDLE_TIME: &str = "CONN_IDLE_TIME";
const MONGODB_RETRY_WRITES: &str = "MONGODB_RETRY_WRITES";
const MONGODB_RETRY_READS: &str = "MONGODB_RETRY_READS";
const POSTGRES_URI: &str = "POSTGRES_URI";

// A static OnceCell to ensure .env is loaded only once
static ENV_LOADER: OnceCell<Option<PathBuf>> = OnceCell::new();

#[derive(Debug)]
pub struct MongoDbConfig {
    pub uri: String,
    pub database_name: String,
    pub max_pool_size: u32,
    pub min_pool_size: u32,
    pub conn_idle_time_secs: u64,
    pub timeout_secs: u64,
    pub retry_reads: bool,
    pub retry_writes: bool,
    pub replica_set: Option<String>,
}
#[derive(Debug)]
pub struct PostgresConfig {
    pub uri: String,
    pub max_pool_size: u32,
    pub min_pool_size: u32,
    pub conn_idle_time_secs: u64,
    pub timeout_secs: u64,
}

#[derive(Debug)]
pub struct Config;

impl Config {
    pub fn build() -> Self {
        // Ensure .env is loaded only once
        ENV_LOADER.get_or_init(|| dotenv().ok());
        Self
    }
    pub fn build_mongodb_config(&self) -> MongoDbConfig {
        let max_pool_size: u32 = get_env_num(MAX_POOL_SIZE, "5");

        let min_pool_size: u32 = get_env_num(MIN_POOL_SIZE, "5");

        let conn_idle_time_secs: u64 = get_env_num(CONN_IDLE_TIME, "30");

        let timeout_secs: u64 = get_env_num(TIMEOUT, "30");

        let replica_set =
            var(MONGODB_REPLICA_SET).map_or(None, |v| if v.len() == 0 { None } else { Some(v) });

        MongoDbConfig {
            uri: var(MONGODB_URI).expect(format!("{} is not set", MONGODB_URI).as_str()),
            database_name: var(MONGODB_NAME)
                .expect(format!("{} is not set", MONGODB_NAME).as_str()),
            max_pool_size,
            min_pool_size,
            conn_idle_time_secs,
            timeout_secs,
            retry_reads: get_env_bool(MONGODB_RETRY_READS, "true"),
            retry_writes: get_env_bool(MONGODB_RETRY_WRITES, "true"),
            replica_set,
        }
    }
    pub fn build_postgres_config(&self) -> PostgresConfig {
        let max_pool_size: u32 = get_env_num(MAX_POOL_SIZE, "5");

        let min_pool_size: u32 = get_env_num(MIN_POOL_SIZE, "5");
        let conn_idle_time_secs: u64 = get_env_num(CONN_IDLE_TIME, "30");

        let timeout_secs: u64 = get_env_num(TIMEOUT, "30");

        PostgresConfig {
            uri: var(POSTGRES_URI).expect(format!("{} is not set", POSTGRES_URI).as_str()),
            max_pool_size,
            min_pool_size,
            conn_idle_time_secs,
            timeout_secs,
        }
    }
}

mod utils {
    use std::env::var;
    use std::fmt::Debug;
    use std::str::FromStr;

    pub trait IsAllowedNum {}

    impl IsAllowedNum for u32 {}
    impl IsAllowedNum for u64 {}

    pub fn get_env_num<T>(name: &str, fallback: &str) -> T
    where
        T: IsAllowedNum + FromStr,
        <T as FromStr>::Err: Debug,
    {
        let fallback = fallback.to_string();
        let value = var(name).map_or(
            fallback.clone(),
            |v| if v.len() == 0 { fallback } else { v },
        );
        let value: T = value
            .parse()
            .expect(format!("{} must be a number", name).as_str());
        value
    }

    pub fn get_env_bool(name: &str, fallback: &str) -> bool {
        let fallback: String = fallback.into();
        let value = var(name).map_or(
            fallback.clone(),
            |v| if v.len() == 0 { fallback } else { v },
        );
        let value: bool = value
            .parse()
            .expect(format!("{} must be a bool", name).as_str());
        value
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn build_config_works() {
        let config = Config::build();
        let _mongo_config = config.build_mongodb_config();
        let _postgres_config = config.build_postgres_config();
    }
}
