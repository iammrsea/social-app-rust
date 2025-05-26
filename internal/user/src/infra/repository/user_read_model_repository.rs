use crate::domain::result::UserDomainResult;
use crate::domain::user_read_model::{GetUsersOptions, GetUsersResult, UserReadModel};

use crate::infra::mongoimpl::user_read_model_repository::MongoUserReadModelRepository;

#[cfg(test)]
use super::user_read_model_repository_trait::MockUserReadModelRepositoryTrait;
#[cfg(test)]
use super::user_read_model_repository_trait::UserReadModelRepositoryTrait;

pub enum UserReadModelRepository {
    MongoDb(MongoUserReadModelRepository),

    #[cfg(test)]
    Mock(MockUserReadModelRepositoryTrait),
}

impl UserReadModelRepository {
    pub async fn get_users(&self, opts: &GetUsersOptions) -> UserDomainResult<GetUsersResult> {
        match self {
            UserReadModelRepository::MongoDb(repo) => repo.get_users(opts).await,
            #[cfg(test)]
            UserReadModelRepository::Mock(repo) => repo.get_users(opts).await,
        }
    }

    pub async fn get_user_by_id(&self, id: &str) -> UserDomainResult<Option<UserReadModel>> {
        match self {
            UserReadModelRepository::MongoDb(repo) => repo.get_user_by_id(id).await,
            #[cfg(test)]
            UserReadModelRepository::Mock(repo) => repo.get_user_by_id(id).await,
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> UserDomainResult<Option<UserReadModel>> {
        match self {
            UserReadModelRepository::MongoDb(repo) => repo.get_user_by_email(email).await,
            #[cfg(test)]
            UserReadModelRepository::Mock(repo) => repo.get_user_by_email(email).await,
        }
    }
}
