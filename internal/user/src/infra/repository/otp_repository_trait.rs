use crate::domain::user_auth::{otp::OtpEntry, result::UserAuthResult};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait OtpRepositoryTrait {
    async fn get_otp_by_user_email(&self, email: &str) -> UserAuthResult<Option<OtpEntry>>;
    async fn upsert_otp<'a>(
        &self,
        otp: OtpEntry,
        tx: Option<shared::db_transactions::DBTransaction<'a>>,
    ) -> UserAuthResult<()>;
    async fn update_opt(&self, otp: OtpEntry) -> UserAuthResult<()>;
    async fn delete_otp(&self, email: &str) -> UserAuthResult<()>;
}
