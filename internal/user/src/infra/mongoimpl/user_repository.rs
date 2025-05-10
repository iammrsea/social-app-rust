use async_trait::async_trait;
use mongodb::{Collection, Database};
use shared::types::AppResult;

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
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn create_account(&self, user: User) -> AppResult<()> {
        unimplemented!()
    }

    async fn make_moderator(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn change_username(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn award_badge(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn revoke_badge(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn ban_user(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn unban_user(&self, user_id: &str, update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn get_user_by_id(&self, user_id: &str) -> AppResult<Option<User>> {
        unimplemented!()
    }

    async fn user_exists(&self, username: &str, email: &str) -> AppResult<bool> {
        unimplemented!()
    }
}
