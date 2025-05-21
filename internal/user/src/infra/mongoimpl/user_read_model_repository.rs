use async_trait::async_trait;
use bson::{Document, doc};
use chrono::{DateTime, Utc};
use futures::stream::StreamExt;
use mongodb::{Collection, Database, options::FindOptions};

use crate::domain::{
    errors::{UserDomainError, UserDomainResult},
    user_read_model::UserReadModel,
    user_read_model_repository::{
        GetUsersOptions, GetUsersResult, SortDirection, UserReadModelRepository,
    },
};

use super::user_document::UserDocument;

pub struct MongoUserReadModelRepository {
    collection: Collection<UserDocument>,
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
    async fn get_users(&self, opts: &GetUsersOptions) -> UserDomainResult<GetUsersResult> {
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
                .map_err(|e| UserDomainError::Internal(e.to_string()))?
                .with_timezone(&Utc);
            let op = if sort_value == 1 { "$gt" } else { "$lt" };
            filter.insert("created_at", doc! {op: created_at});
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
    async fn get_user_by_id(&self, id: &str) -> UserDomainResult<Option<UserReadModel>> {
        let doc = self
            .collection
            .find_one(doc! {"_id": id})
            .await?
            .map(|doc| doc.into());
        Ok(doc)
    }
    async fn get_user_by_email(&self, email: &str) -> UserDomainResult<Option<UserReadModel>> {
        let doc = self
            .collection
            .find_one(doc! {"email": email})
            .await?
            .map(|doc| doc.into());
        Ok(doc)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user::User;

    use super::*;
    use chrono::Duration;
    use shared::test_utils;
    use utils::{insert_many_users, insert_user};
    use uuid::Uuid;

    #[tokio::test]
    async fn get_user_by_id() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);
        insert_user(db.clone(), user.clone()).await;
        let user_repo = MongoUserReadModelRepository::new(db.clone());
        let user_from_db = user_repo.get_user_by_id(user.id()).await.unwrap().unwrap();
        assert_eq!(user_from_db.id, user.id());
    }

    #[tokio::test]
    async fn get_user_by_email() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);
        insert_user(db.clone(), user.clone()).await;
        let user_repo = MongoUserReadModelRepository::new(db.clone());
        let user_from_db = user_repo
            .get_user_by_email(user.email())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user_from_db.email, user.email());
    }

    #[tokio::test]
    async fn get_users() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        insert_many_users(db.clone(), 20).await;
        let user_repo = MongoUserReadModelRepository::new(db.clone());
        let opts = GetUsersOptions {
            first: 10,
            after: None,
            sort_direction: SortDirection::ASC,
        };
        let result = user_repo.get_users(&opts).await.unwrap();
        assert_eq!(result.users.len(), 10);
        assert_eq!(true, result.has_next);
    }

    #[tokio::test]
    async fn get_users_after_cursor_works_asc() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        insert_many_users(db.clone(), 10).await;
        let user_repo = MongoUserReadModelRepository::new(db.clone());
        let after = (Utc::now() + Duration::hours(1)).to_rfc3339();
        let opts = GetUsersOptions {
            first: 10,
            after: Some(after.clone()),
            sort_direction: SortDirection::ASC,
        };
        let result = user_repo.get_users(&opts).await.unwrap();

        assert_eq!(8, result.users.len());
        assert_eq!(false, result.has_next);
    }

    #[tokio::test]
    async fn get_users_after_cursor_works_desc() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        insert_many_users(db.clone(), 10).await;
        let user_repo = MongoUserReadModelRepository::new(db.clone());
        let after = (Utc::now() + Duration::hours(1)).to_rfc3339();
        let opts = GetUsersOptions {
            first: 10,
            after: Some(after.clone()),
            sort_direction: SortDirection::DESC,
        };
        let result = user_repo.get_users(&opts).await.unwrap();

        assert_eq!(2, result.users.len());
        assert_eq!(false, result.has_next);
    }

    mod utils {
        use chrono::Duration;
        use shared::guards::roles::UserRole;

        use crate::domain::user::EmailStatus;

        use super::*;

        pub async fn insert_user(db: Database, user: User) {
            let c: Collection<UserDocument> = db.collection("users");
            let user: UserDocument = user.into();
            c.insert_one(user).await.unwrap();
        }

        pub async fn insert_many_users(db: Database, num: usize) {
            let c: Collection<UserDocument> = db.collection("users");
            let mut users: Vec<User> = Vec::new();

            for i in 0..num {
                let user = User::new_with_all_fields(
                    Uuid::new_v4().to_string(),
                    format!("test-{}@gmail.com", Uuid::new_v4().to_string()),
                    format!("{}-test-{}", Uuid::new_v4().to_string(), i),
                    UserRole::Regular,
                    Utc::now() + Duration::hours(i as i64),
                    None,
                    Utc::now() + Duration::hours(i as i64),
                    vec![],
                    EmailStatus::Verified,
                );
                users.push(user);
            }
            let users: Vec<UserDocument> = users.into_iter().map(|u| u.into()).collect();
            c.insert_many(users).await.unwrap();
        }
    }
}
