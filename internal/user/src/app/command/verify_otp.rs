use std::sync::Arc;

use async_graphql::InputObject;
use async_trait::async_trait;

use crate::{
    domain::user_auth::{errors::UserAuthError, jwt, otp::ComparedOtps},
    infra::repository::otp_repository::OtpRepository,
};
use serde::Deserialize;
use validator::Validate;

use shared::{auth::AppContext, command_handler::CommandHanlder};

use crate::domain::{
    errors::UserDomainError, result::UserDomainResult, user_auth::otp::utils as otp_utils,
};
use crate::infra::repository::user_repository::UserRepository;

#[derive(Debug, Clone, Validate, Deserialize, InputObject)]
pub struct VerifyOtp {
    pub email: String,
    pub otp: String,
}

pub struct VerifyOtpHandler {
    user_repo: Arc<UserRepository>,
    otp_repo: Arc<OtpRepository>,
}

impl VerifyOtpHandler {
    pub fn new(user_repo: Arc<UserRepository>, otp_repo: Arc<OtpRepository>) -> Self {
        Self {
            user_repo,
            otp_repo,
        }
    }
}

#[async_trait]
impl CommandHanlder<VerifyOtp, UserDomainError, String> for VerifyOtpHandler {
    async fn handle(&self, _ctx: &AppContext, cmd: VerifyOtp) -> UserDomainResult<String> {
        let mut otp_entry = self
            .otp_repo
            .get_otp_by_user_email(&cmd.email)
            .await?
            .ok_or(UserAuthError::OtpNotFound)?;

        let user = self
            .user_repo
            .get_user_by_username_or_email("", &cmd.email)
            .await?
            .ok_or(UserDomainError::UserNotFound)?;

        if let Err(err) = otp_entry.validate_otp() {
            if err == UserAuthError::TooManyAttempts {
                //TODO: set up eventing system to delete OTP after too many attempts
                self.otp_repo.delete_otp(&cmd.email).await?;
                return Err(err.into());
            }
            otp_entry.increment_attempts();
            self.otp_repo.upsert_otp(otp_entry, None).await?;
            return Err(err.into());
        }

        if otp_utils::compare_otps(&cmd.otp, otp_entry.otp_hash()) == ComparedOtps::NotEqual {
            otp_entry.increment_attempts();
            self.otp_repo.upsert_otp(otp_entry, None).await?; // TODO: Perhaps, handle this update via eventing system
            return Err(UserAuthError::InvalidOtp.into());
        }

        otp_entry.mark_as_used();
        otp_entry.increment_attempts();
        self.otp_repo.upsert_otp(otp_entry, None).await?;
        let token = jwt::create_jwt(user.email().to_string(), user.role().to_owned())?;
        //TODO: Set up eventing system to delete OTP after successful otp verification
        tracing::info!(
            "OTP verified successfully for user: {}, token: {}",
            user.email(),
            token
        );
        Ok(token)
    }
}
