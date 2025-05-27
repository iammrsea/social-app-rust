use std::sync::Arc;

use async_graphql::InputObject;
use async_trait::async_trait;

use crate::{
    domain::user_auth::{errors::UserAuthError, otp::ComparedOtps},
    infra::repository::otp_repository::OtpRepository,
};
use serde::Deserialize;
use validator::Validate;

use shared::{
    auth::{AppContext, jwt},
    command_handler::CommandHanlder,
    db_transactions::MockTransaction,
};

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
                    let token = jwt::create_jwt(
                        user.email().to_string(),
                        user.role().to_owned(),
                        user.id().to_string(),
                    )?;
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
                Ok(Some("token".to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::{EmailStatus, User};
    use crate::domain::user_auth::otp::{OtpEntry, utils as otp_utils};
    use crate::infra::repository::otp_repository_trait::MockOtpRepositoryTrait;
    use crate::infra::repository::user_repository_trait::MockUserRepositoryTrait;
    use mockall::predicate::eq;
    use shared::auth::{AppContext, AuthUser};
    use shared::guards::roles::UserRole;
    use std::sync::Arc;

    #[tokio::test]
    async fn verify_email_with_otp_success() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_otp_repo = MockOtpRepositoryTrait::new();

        let email = "test@example.com".to_string();
        let otp = "123456".to_string();
        let otp_entry = OtpEntry::new(
            email.clone(),
            false,
            0,
            otp_utils::hash_otp(&otp),
            otp_utils::get_otp_expiration(),
        );
        let mut user = User::new_test_user(None);
        user.set_email_status(EmailStatus::Unverified);

        mock_otp_repo
            .expect_get_otp_by_user_email()
            .with(eq(email.clone()))
            .returning(move |_| Ok(Some(otp_entry.clone())));

        mock_user_repo
            .expect_get_user_by_username_or_email()
            .with(eq("".to_string()), eq(email.clone()))
            .returning(move |_, _| Ok(Some(user.clone())));

        mock_otp_repo.expect_upsert_otp().returning(|_, _| Ok(()));

        mock_user_repo.expect_upsert_user().returning(|_, _| Ok(()));

        let handler = VerifyEmailWithOtpHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let cmd = VerifyEmailWithOtp { email, otp };

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn verify_email_with_otp_invalid_otp() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_otp_repo = MockOtpRepositoryTrait::new();

        let email = "test@example.com".to_string();
        let otp = "wrong_otp".to_string();
        let otp_entry = OtpEntry::new(
            email.clone(),
            false,
            0,
            otp_utils::hash_otp("123456"),
            otp_utils::get_otp_expiration(),
        );
        let mut user = User::new_test_user(None);
        user.set_email_status(EmailStatus::Unverified);

        mock_otp_repo
            .expect_get_otp_by_user_email()
            .with(eq(email.clone()))
            .returning(move |_| Ok(Some(otp_entry.clone())));

        mock_user_repo
            .expect_get_user_by_username_or_email()
            .with(eq("".to_string()), eq(email.clone()))
            .returning(move |_, _| Ok(Some(user.clone())));

        mock_otp_repo.expect_upsert_otp().returning(|_, _| Ok(()));

        let handler = VerifyEmailWithOtpHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let cmd = VerifyEmailWithOtp { email, otp };

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }
}
