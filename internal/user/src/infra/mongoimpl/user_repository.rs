use async_trait::async_trait;
use mongodb::{Collection, Database, bson::doc};
use shared::{errors::user::UserDomainError, types::AppResult};

use crate::domain::{
    user::User,
    user_repository::{F, UserRepository},
};

use super::user_document::UserDocument;

pub struct MongoUserRepository {
    collection: Collection<UserDocument>,
}

impl MongoUserRepository {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }
    async fn find_and_update_user(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        if let Some(user) = self.collection.find_one(doc! {"_id": user_id }).await? {
            let mut domain_user: User = user.into();
            update_fn(&mut domain_user);
            let user: UserDocument = domain_user.into();
            self.collection
                .replace_one(doc! {"_id": user_id}, &user)
                .await?;
            Ok(())
        } else {
            Err(UserDomainError::UserNotFound.into())
        }
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn create_account(&self, _user: User) -> AppResult<()> {
        unimplemented!()
    }

    async fn make_moderator(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    async fn change_username(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    async fn award_badge(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    async fn revoke_badge(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    async fn ban_user(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    async fn unban_user(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    async fn get_user_by_id(&self, user_id: &str) -> AppResult<Option<User>> {
        let user = self
            .collection
            .find_one(doc! {"_id": user_id})
            .await?
            .map(|doc| doc.into());
        Ok(user)
    }

    async fn user_exists(&self, username: &str, email: &str) -> AppResult<bool> {
        let filter = doc! {
            "or": [
                {"email": email},
                {"username": username}
            ]
        };
        let user = self.collection.find_one(filter).await?;
        Ok(user.is_some())
    }
}
