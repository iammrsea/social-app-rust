use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::stream::StreamExt;
use mongodb::{
    Collection, Database,
    bson::{Bson, Document, doc},
    options::FindOptions,
};

use shared::{errors::app::AppError, types::AppResult};

use crate::domain::{
    user_read_model::UserReadModel,
    user_read_model_repository::{
        GetUsersOptions, GetUsersResult, SortDirection, UserReadModelRepository,
    },
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
    async fn get_users(&self, opts: &GetUsersOptions) -> AppResult<GetUsersResult> {
        let mut find_options = FindOptions::default();
        find_options.limit = Some((opts.first + 1) as i64);

        // Set sort direction
        let sort_value = match opts.sort_direction {
            SortDirection::ASC => 1,
            SortDirection::DESC => -1,
        };
        find_options.sort = Some(doc! { "created_at": sort_value });

        // Build filter
        let mut filter = Document::new();

        if let Some(after) = &opts.after {
            let created_at = DateTime::parse_from_rfc3339(&after)
                .map_err(|e| AppError::Internal(e.to_string()))?
                .with_timezone(&Utc);
            let op = if sort_value == 1 { "$gt" } else { "$lt" };
            let created_at = mongodb::bson::DateTime::from_millis(created_at.timestamp_millis());
            filter.insert("created_at", doc! {op: Bson::DateTime(created_at)});
        };

        let mut cursor = self
            .collection
            .find(filter)
            .with_options(find_options)
            .await?;

        let mut users = Vec::new();
        while let Some(doc) = cursor.next().await {
            let doc = doc?;
            let user: UserReadModel = doc.into();
            users.push(user);
        }
        let has_next = users.len() > opts.first as usize;
        if has_next {
            users.truncate(opts.first as usize); // Remove the last element
        }
        let result = GetUsersResult { users, has_next };
        Ok(result)
    }
    async fn get_user_by_id(&self, id: &str) -> AppResult<Option<UserReadModel>> {
        let doc = self
            .collection
            .find_one(doc! {"_id": id})
            .await?
            .map(|doc| doc.into());
        Ok(doc)
    }
    async fn get_user_by_email(&self, email: &str) -> AppResult<Option<UserReadModel>> {
        let doc = self
            .collection
            .find_one(doc! {"email": email})
            .await?
            .map(|doc| doc.into());
        Ok(doc)
    }
}
