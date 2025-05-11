use async_trait::async_trait;
use mongodb::{Collection, Database};
use shared::types::AppResult;

use crate::domain::{
    user_read_model::UserReadModel,
    user_read_model_repository::{GetUsersOptions, GetUsersResult, UserReadModelRepository},
};

use super::user_document::UserDocument;

pub struct MongoUserReadModelRepository {
    pub collection: Collection<UserDocument>, // TODO: remove pub from collection field
}

impl MongoUserReadModelRepository {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }
}

#[async_trait]
impl UserReadModelRepository for MongoUserReadModelRepository {
    async fn get_users(&self, _opts: &GetUsersOptions) -> AppResult<GetUsersResult> {
        unimplemented!()
    }
    async fn get_user_by_id(&self, _id: &str) -> AppResult<Option<UserReadModel>> {
        unimplemented!()
    }
    async fn get_user_by_email(&self, _email: &str) -> AppResult<Option<UserReadModel>> {
        unimplemented!()
    }
}
