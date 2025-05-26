use crate::result::AuthResult;

use super::{OtpEntry, mongoimpl::MongoOtpRepository};

pub enum OtpRepository {
    MongoDb(MongoOtpRepository),
    Mock(MockOtpRepositoryTrait),
}

impl OtpRepository {
    pub async fn get_otp_by_user_email(&self, email: &str) -> AuthResult<OtpEntry> {
        match self {
            OtpRepository::MongoDb(repo) => repo.get_otp_by_user_email(email).await,
            OtpRepository::Mock(mock) => mock.get_otp_by_user_email(email).await,
        }
    }

    pub async fn upsert_otp(&self, otp: OtpEntry) -> AuthResult<()> {
        match self {
            OtpRepository::MongoDb(repo) => repo.upsert_otp(otp).await,
            OtpRepository::Mock(mock) => mock.upsert_otp(otp).await,
        }
    }

    pub async fn delete_otp(&self, email: &str) -> AuthResult<()> {
        match self {
            OtpRepository::MongoDb(repo) => repo.delete_otp(email).await,
            OtpRepository::Mock(mock) => mock.delete_otp(email).await,
        }
    }
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait OtpRepositoryTrait: Send + Sync {
    async fn get_otp_by_user_email(&self, email: &str) -> AuthResult<OtpEntry>;
    async fn upsert_otp(&self, otp: OtpEntry) -> AuthResult<()>;
    async fn update_opt(&self, otp: OtpEntry) -> AuthResult<()>;
    async fn delete_otp(&self, email: &str) -> AuthResult<()>;
}
