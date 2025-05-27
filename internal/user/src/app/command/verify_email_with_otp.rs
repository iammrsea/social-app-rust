use std::sync::Arc;

use async_graphql::InputObject;
use async_trait::async_trait;

use crate::{
    domain::user_auth::{errors::UserAuthError, jwt, otp::ComparedOtps},
    infra::repository::otp_repository::OtpRepository,
};
use serde::Deserialize;
use validator::Validate;

use shared::{auth::AppContext, command_handler::CommandHanlder, db_transactions::MockTransaction};

use crate::domain::{
    errors::UserDomainError, result::UserDomainResult, user::EmailStatus,
    user_auth::otp::utils as otp_utils,
};
use crate::infra::repository::user_repository::UserRepository;
use shared::db_transactions::{DBTransaction, RepoDB};

#[derive(Debug, Clone, Validate, Deserialize, InputObject)]
pub struct VerifyEmailWithOtp {
    pub email: String,
    pub otp: String,
}

pub struct VerifyEmailWithOtpHandler {
    user_repo: Arc<UserRepository>,
    otp_repo: Arc<OtpRepository>,
}

impl VerifyEmailWithOtpHandler {
    pub fn new(user_repo: Arc<UserRepository>, otp_repo: Arc<OtpRepository>) -> Self {
        Self {
            user_repo,
            otp_repo,
        }
    }
}

#[async_trait]
impl CommandHanlder<VerifyEmailWithOtp, UserDomainError, Option<String>>
    for VerifyEmailWithOtpHandler
{
    async fn handle(
        &self,
        _ctx: &AppContext,
        cmd: VerifyEmailWithOtp,
    ) -> UserDomainResult<Option<String>> {
        let mut otp_entry = self
            .otp_repo
            .get_otp_by_user_email(&cmd.email)
            .await?
            .ok_or(UserAuthError::OtpNotFound)?;

        let mut user = self
            .user_repo
            .get_user_by_username_or_email("", &cmd.email)
            .await?
            .ok_or(UserDomainError::UserNotFound)?;

        otp_entry.validate_otp()?;

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
            self.otp_repo.upsert_otp(otp_entry, None).await?;
            return Err(UserAuthError::InvalidOtp.into());
        }

        let repo_db = self.user_repo.get_repo_db();

        match repo_db {
            RepoDB::MongoDb(db) => {
                let mut session = db.client().start_session().await?;
                session.start_transaction().await?;
                let result: UserDomainResult<()> = (|| async {
                    otp_entry.mark_as_used();
                    otp_entry.increment_attempts();
                    self.otp_repo
                        .upsert_otp(otp_entry, Some(DBTransaction::MongoDb(&mut session)))
                        .await?;
                    user.set_email_status(EmailStatus::Verified);
                    self.user_repo
                        .upsert_user(user.clone(), Some(DBTransaction::MongoDb(&mut session)))
                        .await?;
                    Ok(())
                })()
                .await;

                if let Ok(..) = result {
                    session.commit_transaction().await?;
                    let token = jwt::create_jwt(user.email().to_string(), user.role().to_owned())?;
                    tracing::info!(
                        "OTP verified successfully for user: {}, token: {}",
                        user.email(),
                        token
                    );
                    //TODO: set up eventing system to delete OTP after successful  email verification
                    return Ok(Some(token));
                } else {
                    session.abort_transaction().await?;
                }
                Ok(None)
            }
            RepoDB::Mock => {
                self.user_repo
                    .upsert_user(user, Some(DBTransaction::Mock(&mut MockTransaction)))
                    .await?;
                self.otp_repo
                    .upsert_otp(otp_entry, Some(DBTransaction::Mock(&mut MockTransaction)))
                    .await?;
                Ok(None)
            }
        }
    }
}
