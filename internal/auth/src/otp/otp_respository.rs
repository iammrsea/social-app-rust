use async_trait::async_trait;

use crate::result::AuthResult;

use super::OtpEntry;

#[mockall::automock]
#[async_trait]
pub trait OtpRepository: Send + Sync {
    async fn get_otp_by_user_email(&self, email: &str) -> AuthResult<OtpEntry>;
    async fn upsert_otp(&self, otp: OtpEntry) -> AuthResult<()>;
    async fn update_opt(&self, otp: OtpEntry) -> AuthResult<()>;
    async fn delete_otp(&self, email: &str) -> AuthResult<()>;
}
