use std::sync::Arc;

use async_graphql::InputObject;
use async_trait::async_trait;

use crate::infra::repository::otp_repository::OtpRepository;
use serde::Deserialize;
use validator::Validate;

use shared::{
    auth::AppContext, command_handler::CommandHanlder, db_transactions::MockTransaction,
    guards::roles::UserRole,
};

use crate::domain::{
    errors::UserDomainError,
    result::UserDomainResult,
    user::{EmailStatus, User},
    user_auth::otp::{OtpEntry, utils as otp_utils},
};
use crate::infra::repository::user_repository::UserRepository;
use shared::db_transactions::{DBTransaction, RepoDB};

#[derive(Debug, Clone, Validate, Deserialize, InputObject)]
pub struct SignUp {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 40))]
    pub username: String,
}

pub struct SignUpHandler {
    user_repo: Arc<UserRepository>,
    otp_repo: Arc<OtpRepository>,
}

impl SignUpHandler {
    pub fn new(user_repo: Arc<UserRepository>, otp_repo: Arc<OtpRepository>) -> Self {
        Self {
            user_repo,
            otp_repo,
        }
    }
}

#[async_trait]
impl CommandHanlder<SignUp, UserDomainError> for SignUpHandler {
    async fn handle(&self, _ctx: &AppContext, cmd: SignUp) -> UserDomainResult<()> {
        cmd.validate()?;
        let user = self
            .user_repo
            .get_user_by_username_or_email(&cmd.username, &cmd.email)
            .await?;

        if user.is_some() && user.as_ref().unwrap().email_status() == &EmailStatus::Verified {
            return Err(UserDomainError::UsernameOrEmailTaken);
        }

        let user_email = cmd.email.clone();
        let new_user = User::new(cmd.email, cmd.username, UserRole::Regular);

        let user = user.map_or(new_user, |existing_user| existing_user);

        let otp_val = otp_utils::generate_otp();
        let otp_hash = otp_utils::hash_otp(&otp_val);
        let expires_at = otp_utils::get_otp_expiration();
        let otp_entry = OtpEntry::new(user_email.clone(), false, 0, otp_hash, expires_at);

        let repo_db = self.user_repo.get_repo_db();
        match repo_db {
            RepoDB::MongoDb(db) => {
                let mut session = db.client().start_session().await?;
                session.start_transaction().await?;
                let result: UserDomainResult<()> = (|| async {
                    self.user_repo
                        .upsert_user(user, Some(DBTransaction::MongoDb(&mut session)))
                        .await?;

                    self.otp_repo
                        .upsert_otp(otp_entry, Some(DBTransaction::MongoDb(&mut session)))
                        .await?;

                    Ok(())
                })()
                .await;

                if let Ok(..) = result {
                    session.commit_transaction().await?;
                    //TODO: send otp to user via email or sms
                    tracing::info!("OTP for user {} is {}", user_email, otp_val);
                    Ok(())
                } else {
                    session.abort_transaction().await?;
                    return result;
                }
            }
            RepoDB::Mock => {
                self.user_repo
                    .upsert_user(user, Some(DBTransaction::Mock(&mut MockTransaction)))
                    .await?;
                self.otp_repo
                    .upsert_otp(otp_entry, Some(DBTransaction::Mock(&mut MockTransaction)))
                    .await?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::repository::{
        otp_repository::OtpRepository, otp_repository_trait::MockOtpRepositoryTrait,
    };
    use mockall::predicate::eq;
    use shared::auth::{AppContext, AuthUser};
    use std::sync::Arc;

    use crate::infra::repository::user_repository_trait::MockUserRepositoryTrait;

    #[tokio::test]
    async fn test_sign_up_success() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_otp_repo = MockOtpRepositoryTrait::new();

        let cmd = SignUp {
            email: "test@gmail.com".into(),
            username: "test".into(),
        };

        let expected_username = cmd.username.clone();
        let expected_email = cmd.email.clone();
        let expected_email_opt = cmd.email.clone();

        let auth_user = AuthUser::new_test_auth_user(UserRole::Guest);

        mock_user_repo
            .expect_get_user_by_username_or_email()
            .with(
                eq(expected_username.to_string()),
                eq(expected_email.to_string()),
            )
            .returning(|_, _| Ok(None));

        mock_user_repo
            .expect_upsert_user()
            .withf(move |user, session| {
                session.is_some() && {
                    user.username() == expected_username && user.email() == expected_email
                }
            })
            .returning(|_, _| Ok(()));
        mock_otp_repo
            .expect_upsert_otp()
            .withf(move |otp, session| {
                session.is_some() && {
                    otp.email() == &expected_email_opt && !otp.is_used() && *otp.attempts() == 0
                }
            })
            .returning(|_, _| Ok(()));

        let handler = SignUpHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );
        let ctx = AppContext::new().with_user(auth_user);
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_sign_up_failed_email_or_username_exists() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mut mock_otp_repo = MockOtpRepositoryTrait::new();

        let cmd = SignUp {
            email: "test@gmail.com".into(),
            username: "test".into(),
        };

        let expected_username = cmd.username.clone();
        let expected_email = cmd.email.clone();

        let auth_user = AuthUser::new_test_auth_user(UserRole::Guest);

        mock_user_repo
            .expect_get_user_by_username_or_email()
            .with(
                eq(expected_username.to_string()),
                eq(expected_email.to_string()),
            )
            .returning(|_, _| Ok(Some(User::new_test_user(None))));

        mock_user_repo.expect_upsert_user().never();
        mock_otp_repo.expect_upsert_otp().never();

        let handler = SignUpHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );
        let ctx = AppContext::new().with_user(auth_user);
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn validation_errors() {
        let mock_user_repo = MockUserRepositoryTrait::new();
        let mock_otp_repo = MockOtpRepositoryTrait::new();

        let cmd = SignUp {
            email: "invalid_email".into(),
            username: "us".into(),
        };
        let handler = SignUpHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            Arc::new(OtpRepository::Mock(mock_otp_repo)),
        );
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err(), "Expected validation error");
    }
}
