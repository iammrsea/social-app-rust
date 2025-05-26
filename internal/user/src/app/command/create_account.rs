use std::sync::Arc;

use async_graphql::InputObject;
use async_trait::async_trait;

use auth::repository::OtpRepository;
use serde::Deserialize;
use validator::Validate;

use shared::{auth::AppContext, command_handler::CommandHanlder, guards::roles::UserRole};

use crate::infra::repository::db_transactions::RepoDB;
use crate::infra::repository::user_repository::UserRepository;
use crate::{
    domain::{
        errors::UserDomainError,
        result::UserDomainResult,
        user::{EmailStatus, User},
    },
    infra::repository::db_transactions::DBTransaction,
};

#[derive(Debug, Clone, Validate, Deserialize, InputObject)]
pub struct CreateAccount {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 40))]
    pub username: String,
}

pub struct CreateAccountHandler {
    user_repo: Arc<UserRepository>,
    pub otp_repo: OtpRepository, // TODO: remove pub modifier
}

impl CreateAccountHandler {
    pub fn new(user_repo: Arc<UserRepository>, otp_repo: OtpRepository) -> Self {
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

        let repo_db = self.user_repo.get_repo_db();
        let user = User::new(cmd.username, cmd.email, UserRole::Regular);
        match repo_db {
            RepoDB::MongoDb(db) => {
                let mut session = db.client().start_session().await?;
                session.start_transaction().await?;
                let result: UserDomainResult<()> = (|| async {
                    self.user_repo
                        .upsert_user(user, Some(DBTransaction::MongoDb(&mut session)))
                        .await?;
                    Ok(())
                })()
                .await;

                match result {
                    Ok(_) => {
                        session.commit_transaction().await?;
                    }
                    Err(..) => {
                        session.abort_transaction().await?;
                    }
                }
                result
            }
            RepoDB::Mock => {
                self.user_repo.upsert_user(user, None).await?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use auth::repository::{MockOtpRepositoryTrait, OtpRepository};
    use mockall::predicate::eq;
    use shared::auth::{AppContext, AuthUser};
    use std::sync::Arc;

    use crate::infra::repository::user_repository_trait::MockUserRepositoryTrait;

    #[tokio::test]
    async fn create_account_success() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mock_otp_repo = MockOtpRepositoryTrait::new();

        let cmd = CreateAccount {
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
            .returning(|_, _| Ok(None));

        mock_user_repo.expect_upsert_user().returning(|_, _| Ok(()));

        mock_user_repo
            .expect_create_account()
            .returning(move |user| {
                assert_eq!(expected_email.to_string(), user.email());
                assert_eq!(expected_username.to_string(), user.username());
                Ok(())
            });

        let handler = CreateAccountHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            OtpRepository::Mock(mock_otp_repo),
        );
        let ctx = AppContext::new().with_user(auth_user);
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn create_account_failed_email_or_username_exists() {
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let mock_otp_repo = MockOtpRepositoryTrait::new();

        let cmd = CreateAccount {
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

        mock_user_repo.expect_create_account().never();

        let handler = CreateAccountHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            OtpRepository::Mock(mock_otp_repo),
        );
        let ctx = AppContext::new().with_user(auth_user);
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn validation_errors() {
        let mock_user_repo = MockUserRepositoryTrait::new();
        let mock_otp_repo = MockOtpRepositoryTrait::new();

        let cmd = CreateAccount {
            email: "invalid_email".into(),
            username: "us".into(),
        };
        let handler = CreateAccountHandler::new(
            Arc::new(UserRepository::Mock(mock_user_repo)),
            OtpRepository::Mock(mock_otp_repo),
        );
        let ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));
        let result = handler.handle(&ctx, cmd).await;
        assert!(result.is_err(), "Expected validation error");
    }
}
