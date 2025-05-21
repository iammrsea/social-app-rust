use std::sync::Arc;

use async_graphql::InputObject;
use async_trait::async_trait;

use auth::otp_respository::OtpRepository;
use serde::Deserialize;
use validator::Validate;

use shared::{auth::AppContext, command_handler::CommandHanlder, guards::roles::UserRole};

use crate::domain::{
    errors::{UserDomainError, UserDomainResult},
    user::{EmailStatus, User},
    user_repository::UserRepository,
};

#[derive(Debug, Clone, Validate, Deserialize, InputObject)]
pub struct CreateAccount {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 40))]
    pub username: String,
}

pub struct CreateAccountHandler {
    user_repo: Arc<dyn UserRepository>,
    otp_repo: Arc<dyn OtpRepository>,
}

impl CreateAccountHandler {
    pub fn new(user_repo: Arc<dyn UserRepository>, otp_repo: Arc<dyn OtpRepository>) -> Self {
        Self {
            user_repo,
            otp_repo,
        }
    }
}

#[async_trait]
impl CommandHanlder<CreateAccount, UserDomainError> for CreateAccountHandler {
    async fn handle(&self, _ctx: &AppContext, cmd: CreateAccount) -> UserDomainResult<()> {
        cmd.validate()?;
        let user = self
            .user_repo
            .get_user_by_username_or_email(&cmd.username, &cmd.email)
            .await?;
        if user.is_some() && user.unwrap().email_status() == &EmailStatus::Verified {
            return Err(UserDomainError::UsernameOrEmailTaken);
        }
        // let exists = self
        //     .repo
        //     .user_exists(&cmd.username, &cmd.email, Some(EmailStatus::Verified))
        //     .await?;
        // if exists {
        //     return Err(UserDomainError::UsernameOrEmailTaken.into());
        // }
        let user = User::new(cmd.email, cmd.username, UserRole::Regular);
        self.user_repo.create_account(user).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use auth::otp_respository::MockOtpRepository;
    use mockall::predicate::eq;
    use shared::auth::{AppContext, AuthUser};
    use std::sync::Arc;

    use crate::domain::user_repository::MockUserRepository;

    #[tokio::test]
    async fn create_account_success() {
        let mut mock_user_repo = MockUserRepository::new();
        let mock_otp_repo = MockOtpRepository::new();

        let cmd = CreateAccount {
            email: "test@gmail.com".into(),
            username: "test".into(),
        };

        let expected_username = cmd.username.clone();
        let expected_email = cmd.email.clone();

        let auth_user = AuthUser::new_test_auth_user(UserRole::Guest);

        mock_user_repo
            .expect_user_exists()
            .with(
                eq(expected_username.to_string()),
                eq(expected_email.to_string()),
                eq(Some(EmailStatus::Verified)),
            )
            .returning(|_, _, _| Ok(false));

        mock_user_repo
            .expect_create_account()
            .returning(move |user| {
                assert_eq!(expected_email.to_string(), user.email());
                assert_eq!(expected_username.to_string(), user.username());
                Ok(())
            });

        let handler = CreateAccountHandler::new(Arc::new(mock_user_repo), Arc::new(mock_otp_repo));
        let ctx = AppContext::new().with_user(auth_user);
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn create_account_failed_email_or_username_exists() {
        let mut mock_user_repo = MockUserRepository::new();
        let mock_otp_repo = MockOtpRepository::new();

        let cmd = CreateAccount {
            email: "test@gmail.com".into(),
            username: "test".into(),
        };

        let expected_username = cmd.username.clone();
        let expected_email = cmd.email.clone();

        let auth_user = AuthUser::new_test_auth_user(UserRole::Guest);

        mock_user_repo
            .expect_user_exists()
            .with(
                eq(expected_username.to_string()),
                eq(expected_email.to_string()),
                eq(Some(EmailStatus::Verified)),
            )
            .returning(|_, _, _| Ok(true));

        mock_user_repo.expect_create_account().never();

        let handler = CreateAccountHandler::new(Arc::new(mock_user_repo), Arc::new(mock_otp_repo));
        let ctx = AppContext::new().with_user(auth_user);
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn validation_errors() {
        let mock_user_repo = MockUserRepository::new();
        let mock_otp_repo = MockOtpRepository::new();

        let cmd = CreateAccount {
            email: "invalid_email".into(),
            username: "us".into(),
        };
        let handler = CreateAccountHandler::new(Arc::new(mock_user_repo), Arc::new(mock_otp_repo));
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err(), "Expected validation error");
    }
}
