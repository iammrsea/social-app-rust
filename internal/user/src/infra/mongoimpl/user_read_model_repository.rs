use async_trait::async_trait;
use mongodb::{Collection, Database};
use shared::types::AppResult;

use crate::domain::{
    user_read_model::UserReadModel,
    user_read_model_repository::{GetUsersOptions, GetUsersResult, UserReadModelRepository},
};

use super::user_document::UserDocument;

pub struct MongoUserReadModelRepository {
    collection: Collection<UserDocument>,
}

impl MongoUserReadModelRepository {
    fn new(db: Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }
}

#[async_trait]
impl UserReadModelRepository for MongoUserReadModelRepository {
    async fn get_users(&self, opts: &GetUsersOptions) -> AppResult<GetUsersResult> {
        unimplemented!()
    }
    async fn get_user_by_id(&self, id: &str) -> AppResult<Option<UserReadModel>> {
        unimplemented!()
    }
    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<UserReadModel>> {
        unimplemented!()
    }
}
