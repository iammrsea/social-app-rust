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
};

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
            //TODO: set up eventing system to delete OTP after too many attempts
            self.otp_repo.delete_otp(&cmd.email).await?;
            return Err(err.into());
        }

        if otp_utils::compare_otps(&cmd.otp, otp_entry.otp_hash()) == ComparedOtps::NotEqual {
            otp_entry.increment_attempts();
            self.otp_repo.upsert_otp(otp_entry, None).await?; // TODO: Perhaps, handle this update via eventing system
            return Err(UserAuthError::MissMatchOtp.into());
        }

        otp_entry.mark_as_used();
        otp_entry.increment_attempts();
        self.otp_repo.upsert_otp(otp_entry, None).await?;
        let token = jwt::create_jwt(
            user.email().to_string(),
            user.role().to_owned(),
            user.id().to_string(),
        )?;
        //TODO: Set up eventing system to delete OTP after successful otp verification
        tracing::info!(
            "OTP verified successfully for user: {}, token: {}",
            user.email(),
            token
        );
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::User;
    use crate::domain::user_auth::otp::{MAX_ALLOWED_ATTEMPTS, OtpEntry, utils as otp_utils};
    use crate::infra::repository::otp_repository_trait::MockOtpRepositoryTrait;
    use crate::infra::repository::user_repository_trait::MockUserRepositoryTrait;
    use mockall::predicate::eq;
    use shared::auth::{AppContext, AuthUser};
    use shared::guards::roles::UserRole;
    use std::sync::Arc;

    #[tokio::test]
    async fn verify_otp_success() {
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
        let user = User::new_test_user(None);

        let mut expected_otp_entry = otp_entry.clone();
        expected_otp_entry.increment_attempts();
        expected_otp_entry.mark_as_used();

        mock_otp_repo
            .expect_get_otp_by_user_email()
            .with(eq(email.clone()))
            .returning(move |_| Ok(Some(otp_entry.clone())));

        mock_user_repo
            .expect_get_user_by_username_or_email()
            .with(eq("".to_string()), eq(email.clone()))
            .returning(move |_, _| Ok(Some(user.clone())));

        mock_otp_repo
            .expect_upsert_otp()
            .withf(move |otp, trx| otp == &expected_otp_entry && trx.is_none())
            .returning(|_, _| Ok(()));

        let handler = VerifyOtpHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let cmd = VerifyOtp { email, otp };

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn verify_otp_miss_match_otp() {
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
        let user = User::new_test_user(None);

        let mut expected_otp_entry = otp_entry.clone();

        mock_otp_repo
            .expect_get_otp_by_user_email()
            .with(eq(email.clone()))
            .returning(move |_| Ok(Some(otp_entry.clone())));

        mock_user_repo
            .expect_get_user_by_username_or_email()
            .with(eq("".to_string()), eq(email.clone()))
            .returning(move |_, _| Ok(Some(user.clone())));

        expected_otp_entry.increment_attempts();
        mock_otp_repo
            .expect_upsert_otp()
            .withf(move |otp, trx| otp == &expected_otp_entry && trx.is_none())
            .returning(|_, _| Ok(()));

        let handler = VerifyOtpHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let cmd = VerifyOtp { email, otp };

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn verify_otp_invalid_otp() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_otp_repo = MockOtpRepositoryTrait::new();

        let email = "test@example.com".to_string();
        let otp = "wrong_otp".to_string();
        let otp_entry = OtpEntry::new(
            email.clone(),
            true,
            MAX_ALLOWED_ATTEMPTS,
            otp_utils::hash_otp("123456"),
            otp_utils::get_otp_expiration(),
        );
        let user = User::new_test_user(None);

        mock_otp_repo
            .expect_get_otp_by_user_email()
            .with(eq(email.clone()))
            .returning(move |_| Ok(Some(otp_entry.clone())));

        mock_user_repo
            .expect_get_user_by_username_or_email()
            .with(eq("".to_string()), eq(email.clone()))
            .returning(move |_, _| Ok(Some(user.clone())));

        mock_otp_repo
            .expect_delete_otp()
            .with(eq(email.clone()))
            .returning(move |_| Ok(()));

        mock_otp_repo.expect_upsert_otp().never();

        let handler = VerifyOtpHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let cmd = VerifyOtp { email, otp };

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }
}
