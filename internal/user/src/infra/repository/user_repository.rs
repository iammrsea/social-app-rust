use crate::infra::mongoimpl::user_repository::MongoUserRepository;

use crate::domain::result::UserDomainResult;
use crate::domain::user::{EmailStatus, User};

use super::db_transactions::{DBTransaction, RepoDB};

#[cfg(test)]
use super::user_repository_trait::UserRepositoryTrait;

pub enum UserRepository {
    MongoDb(MongoUserRepository),
    #[cfg(test)]
    Mock(super::user_repository_trait::MockUserRepositoryTrait),
}

impl UserRepository {
    pub async fn create_account(&self, user: User) -> UserDomainResult<()> {
        match self {
            UserRepository::MongoDb(repo) => repo.create_account(user).await,
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.create_account(user).await,
        }
    }
    pub async fn make_moderator<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        match self {
            UserRepository::MongoDb(repo) => repo.make_moderator(user_id, update_fn).await,
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.make_moderator(user_id, update_fn).await,
        }
    }
    pub async fn change_username<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        match self {
            UserRepository::MongoDb(repo) => repo.change_username(user_id, update_fn).await,
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.change_username(user_id, update_fn).await,
        }
    }
    pub async fn award_badge<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        match self {
            UserRepository::MongoDb(repo) => repo.award_badge(user_id, update_fn).await,
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.award_badge(user_id, update_fn).await,
        }
    }
    pub async fn revoke_badge<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        match self {
            UserRepository::MongoDb(repo) => repo.revoke_badge(user_id, update_fn).await,
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.revoke_badge(user_id, update_fn).await,
        }
    }
    pub async fn ban_user<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        match self {
            UserRepository::MongoDb(repo) => repo.ban_user(user_id, update_fn).await,
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.ban_user(user_id, update_fn).await,
        }
    }
    pub async fn unban_user<F: FnOnce(&mut User) + Send + 'static>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        match self {
            UserRepository::MongoDb(repo) => repo.unban_user(user_id, update_fn).await,
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.unban_user(user_id, update_fn).await,
        }
    }
    pub async fn get_user_by_id(&self, user_id: &str) -> UserDomainResult<Option<User>> {
        match self {
            UserRepository::MongoDb(repo) => repo.get_user_by_id(user_id).await,
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.get_user_by_id(user_id).await,
        }
    }
    pub async fn get_user_by_username_or_email(
        &self,
        username: &str,
        email: &str,
    ) -> UserDomainResult<Option<User>> {
        match self {
            UserRepository::MongoDb(repo) => {
                repo.get_user_by_username_or_email(username, email).await
            }
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.get_user_by_username_or_email(username, email).await,
        }
    }
    pub async fn user_exists(
        &self,
        username: &str,
        email: &str,
        email_status: Option<EmailStatus>,
    ) -> UserDomainResult<bool> {
        match self {
            UserRepository::MongoDb(repo) => repo.user_exists(username, email, email_status).await,
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.user_exists(username, email, email_status).await,
        }
    }
    pub async fn upsert_user<'a>(
        &self,
        user: User,
        session: Option<DBTransaction<'a>>,
    ) -> UserDomainResult<()> {
        match self {
            UserRepository::MongoDb(repo) => repo.upsert_user(user, session).await,
            #[cfg(test)]
            UserRepository::Mock(repo) => repo.upsert_user(user, session).await,
        }
    }
    pub fn get_repo_db(&self) -> RepoDB {
        match self {
            UserRepository::MongoDb(repo) => repo.get_repo_db(),
            #[cfg(test)]
            UserRepository::Mock(..) => RepoDB::Mock,
        }
    }
}
