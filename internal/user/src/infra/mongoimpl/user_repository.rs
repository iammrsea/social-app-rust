use async_trait::async_trait;
use mongodb::{Collection, Database};
use shared::types::AppResult;

use crate::domain::{
    user::User,
    user_repository::{F, UserRepository},
};

use super::user_document::UserDocument;

pub struct MongoUserRepository {
    pub collection: Collection<UserDocument>, //TODO: remove pub from collection field
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
    async fn create_account(&self, _user: User) -> AppResult<()> {
        unimplemented!()
    }

    async fn make_moderator(&self, _user_id: &str, _update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn change_username(&self, _user_id: &str, _update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn award_badge(&self, _user_id: &str, _update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn revoke_badge(&self, _user_id: &str, _update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn ban_user(&self, _user_id: &str, _update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn unban_user(&self, _user_id: &str, _update_fn: F) -> AppResult<()> {
        unimplemented!()
    }

    async fn get_user_by_id(&self, _user_id: &str) -> AppResult<Option<User>> {
        unimplemented!()
    }

    async fn user_exists(&self, _username: &str, _email: &str) -> AppResult<bool> {
        unimplemented!()
    }
}
