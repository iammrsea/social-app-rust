use std::env::var;
use std::fmt::Debug;
use std::path::PathBuf;

use dotenvy::dotenv;
use once_cell::sync::OnceCell;

const AUTH_SECRET: &str = "AUTH_SECRET";
const RUST_ENV: &str = "RUST_ENV";
const PORT: &str = "PORT";

// A static OnceCell to ensure .env is loaded only once
static ENV_LOADER: OnceCell<Option<PathBuf>> = OnceCell::new();

#[derive(Debug)]
pub struct Config {
    pub auth_secret: String,
    pub port: String,
    pub rust_environment: Environment,
}

#[derive(Debug)]
pub enum Environment {
    Development,
    Production,
    Test,
}

impl Into<Environment> for String {
    fn into(self) -> Environment {
        if &self == "production" {
            Environment::Production
        } else if &self == "development" {
            Environment::Development
        } else if &self == "test" {
            Environment::Test
        } else {
            panic!("Invalid value for RUST_ENV variable: {}", self)
        }
    }
}

impl From<Environment> for String {
    fn from(value: Environment) -> Self {
        match value {
            Environment::Test => "test".into(),
            Environment::Production => "production".into(),
            Environment::Development => "development".into(),
        }
    }
}

impl Config {
    pub fn build() -> Self {
        // Ensure .env is loaded only once
        ENV_LOADER.get_or_init(|| dotenv().ok());

        let auth_secret = var(AUTH_SECRET).expect(format!("{} is not set", AUTH_SECRET).as_str());

        let rust_environment = var(RUST_ENV)
            .expect(
                format!(
                    "{} is not set. Allowed values are: production, development and test",
                    RUST_ENV
                )
                .as_str(),
            )
            .into();

        Self {
            auth_secret,
            port: var(PORT).unwrap_or("8080".into()),
            rust_environment,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn build_config_works() {
        let _config = Config::build();
    }
}
