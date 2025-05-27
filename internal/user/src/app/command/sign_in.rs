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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::{EmailStatus, User};
    use crate::infra::repository::otp_repository_trait::MockOtpRepositoryTrait;
    use crate::infra::repository::user_repository_trait::MockUserRepositoryTrait;
    use mockall::predicate::eq;
    use shared::auth::{AppContext, AuthUser};
    use shared::guards::roles::UserRole;
    use std::sync::Arc;

    #[tokio::test]
    async fn sign_in_success() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_otp_repo = MockOtpRepositoryTrait::new();

        let email = "test@example.com".to_string();
        let mut user = User::new_test_user(None);
        user.set_email_status(EmailStatus::Verified);

        mock_user_repo
            .expect_get_user_by_username_or_email()
            .with(eq("".to_string()), eq(email.clone()))
            .returning(move |_, _| Ok(Some(user.clone())));

        mock_otp_repo.expect_upsert_otp().returning(|_, _| Ok(()));

        let handler = SignInHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let cmd = SignIn { email };

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn sign_in_unverified_email() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mock_otp_repo = MockOtpRepositoryTrait::new();

        let email = "test@example.com".to_string();
        let mut user = User::new_test_user(None);
        user.set_email_status(EmailStatus::Unverified);

        mock_user_repo
            .expect_get_user_by_username_or_email()
            .with(eq("".to_string()), eq(email.clone()))
            .returning(move |_, _| Ok(Some(user.clone())));

        let handler = SignInHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let cmd = SignIn { email };

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn sign_in_user_not_found() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mock_otp_repo = MockOtpRepositoryTrait::new();

        let email = "test@example.com".to_string();

        mock_user_repo
            .expect_get_user_by_username_or_email()
            .with(eq("".to_string()), eq(email.clone()))
            .returning(|_, _| Ok(None));

        let handler = SignInHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );

        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Regular));
        let cmd = SignIn { email };

        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err());
    }
}
