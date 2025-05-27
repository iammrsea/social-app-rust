use std::sync::Arc;

use async_graphql::InputObject;
use async_trait::async_trait;

use crate::infra::repository::otp_repository::OtpRepository;
use serde::Deserialize;
use validator::Validate;

use shared::{auth::AppContext, command_handler::CommandHanlder};

use crate::domain::{
    errors::UserDomainError,
    result::UserDomainResult,
    user::EmailStatus,
    user_auth::otp::{OtpEntry, utils as otp_utils},
};
use crate::infra::repository::user_repository::UserRepository;

#[derive(Debug, Clone, Validate, Deserialize, InputObject)]
pub struct SignIn {
    #[validate(email)]
    pub email: String,
}

pub struct SignInHandler {
    user_repo: Arc<UserRepository>,
    otp_repo: Arc<OtpRepository>,
}

impl SignInHandler {
    pub fn new(user_repo: Arc<UserRepository>, otp_repo: Arc<OtpRepository>) -> Self {
        Self {
            user_repo,
            otp_repo,
        }
    }
}

#[async_trait]
impl CommandHanlder<SignIn, UserDomainError> for SignInHandler {
    async fn handle(&self, _ctx: &AppContext, cmd: SignIn) -> UserDomainResult<()> {
        cmd.validate()?;
        let user = self
            .user_repo
            .get_user_by_username_or_email("", &cmd.email)
            .await?
            .ok_or(UserDomainError::UserNotFound)?;

        if user.email_status() == &EmailStatus::Unverified {
            return Err(UserDomainError::UnverifiedEmail);
        }

        let otp_val = otp_utils::generate_otp();
        let otp_hash = otp_utils::hash_otp(&otp_val);
        let expires_at = otp_utils::get_otp_expiration();
        let otp_entry = OtpEntry::new(cmd.email.clone(), false, 0, otp_hash, expires_at);

        self.otp_repo.upsert_otp(otp_entry, None).await?;
        //TODO: send otp to user via email or sms
        tracing::info!("OTP for user {} is {}", user.email(), otp_val);
        Ok(())
    }
}
