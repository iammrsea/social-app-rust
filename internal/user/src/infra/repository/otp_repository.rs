use crate::domain::user_auth::{otp::OtpEntry, result::UserAuthResult};

use crate::infra::mongoimpl::otp_respository::MongoOtpRepository;

#[cfg(test)]
use super::otp_repository_trait::OtpRepositoryTrait;

pub enum OtpRepository {
    MongoDb(MongoOtpRepository),
    #[cfg(test)]
    Mock(super::otp_repository_trait::MockOtpRepositoryTrait),
}

impl OtpRepository {
    pub async fn get_otp_by_user_email(&self, email: &str) -> UserAuthResult<Option<OtpEntry>> {
        match self {
            OtpRepository::MongoDb(repo) => repo.get_otp_by_user_email(email).await,
            #[cfg(test)]
            OtpRepository::Mock(mock) => mock.get_otp_by_user_email(email).await,
        }
    }

    pub async fn upsert_otp<'a>(
        &self,
        otp: OtpEntry,
        tx: Option<shared::db_transactions::DBTransaction<'a>>,
    ) -> UserAuthResult<()> {
        match self {
            OtpRepository::MongoDb(repo) => repo.upsert_otp(otp, tx).await,
            #[cfg(test)]
            OtpRepository::Mock(mock) => mock.upsert_otp(otp, tx).await,
        }
    }

    pub async fn delete_otp(&self, email: &str) -> UserAuthResult<()> {
        match self {
            OtpRepository::MongoDb(repo) => repo.delete_otp(email).await,
            #[cfg(test)]
            OtpRepository::Mock(mock) => mock.delete_otp(email).await,
        }
    }
}
