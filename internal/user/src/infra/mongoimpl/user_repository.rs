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
    async fn create_account(&self, user: User) -> AppResult<()> {
        let user: UserDocument = user.into();
        self.collection.insert_one(user).await?;
        Ok(())
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
            "$or": [
                {"email": email},
                {"username": username}
            ]
        };
        let user = self.collection.find_one(filter).await?;
        Ok(user.is_some())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user::BanType;

    use super::*;
    use shared::test_utils;
    use shared::types::non_empty_string::NonEmptyString;
    use utils::{assert_user_in_db_equals, get_user_from_db, insert_user};
    use uuid::Uuid;

    #[tokio::test]
    async fn create_account() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);
        let user_repo = MongoUserRepository::new(db.clone());
        user_repo.create_account(user.clone()).await.unwrap();
        assert_user_in_db_equals(user, db).await;
    }

    #[tokio::test]
    async fn user_exists() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));

        let user = User::new_test_user(None);
        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        let res = user_repo
            .user_exists(user.username(), user.email())
            .await
            .unwrap();
        assert_eq!(true, res);
    }

    #[tokio::test]
    async fn get_user_by_id() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));

        let user = User::new_test_user(None);
        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        let res = user_repo.get_user_by_id(user.id()).await.unwrap().unwrap();
        let user: UserDocument = user.into();
        let res: UserDocument = res.into();
        assert_eq!(res, user);
    }

    #[tokio::test]
    async fn make_moderator() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let mut user = User::new_test_user(None);

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .make_moderator(
                user.id(),
                Box::new(|u| {
                    u.make_moderator();
                }),
            )
            .await
            .unwrap();
        user.make_moderator();
        assert_user_in_db_equals(user, db).await;
    }

    #[tokio::test]
    async fn ban_user() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .ban_user(
                user.id(),
                Box::new(|u| {
                    let reason = NonEmptyString::new("abuse".into()).unwrap();
                    u.ban(reason, BanType::Indefinite);
                }),
            )
            .await
            .unwrap();
        let user_from_db = get_user_from_db(db.clone(), user.id()).await;
        assert_eq!(true, user_from_db.ban_status.unwrap().is_banned);
    }

    #[tokio::test]
    async fn unban_user() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .unban_user(
                user.id(),
                Box::new(|u| {
                    u.unban();
                }),
            )
            .await
            .unwrap();
        let user_from_db = get_user_from_db(db.clone(), user.id()).await;
        assert_eq!(None, user_from_db.ban_status);
    }

    #[tokio::test]
    async fn award_badge() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .award_badge(
                user.id(),
                Box::new(|u| {
                    u.award_badge(NonEmptyString::new("Helpful".into()).unwrap());
                }),
            )
            .await
            .unwrap();
        let user_from_db = get_user_from_db(db.clone(), user.id()).await;
        assert_eq!(1, user_from_db.badges.len());
    }

    #[tokio::test]
    async fn revoke_badge() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let mut user = User::new_test_user(None);

        let badge = NonEmptyString::new("Helpful".into()).unwrap();
        user.award_badge(badge.clone());

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .revoke_badge(
                user.id(),
                Box::new(|u| {
                    u.revoke_badge(badge);
                }),
            )
            .await
            .unwrap();
        let user_from_db = get_user_from_db(db.clone(), user.id()).await;
        assert_eq!(0, user_from_db.badges.len());
    }

    #[tokio::test]
    async fn change_username() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);

        let username = NonEmptyString::new("new_username".into()).unwrap();

        let expected_username = username.clone();

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .change_username(
                user.id(),
                Box::new(|u| {
                    u.change_username(username);
                }),
            )
            .await
            .unwrap();
        let user_from_db = get_user_from_db(db.clone(), user.id()).await;
        assert_eq!(expected_username.to_string(), user_from_db.username);
    }

    mod utils {
        use super::*;
        pub async fn assert_user_in_db_equals(user: User, db: Database) {
            let user_from_db: UserDocument = db
                .collection("users")
                .find_one(doc! {"_id": user.id()})
                .await
                .unwrap()
                .unwrap();
            let user: UserDocument = user.into();

            assert_eq!(
                user_from_db, user,
                "expected user: {:#?}, got user:
                {:#?}",
                user_from_db, user
            );
        }

        pub async fn insert_user(db: Database, user: User) {
            let c: Collection<UserDocument> = db.collection("users");
            let user: UserDocument = user.into();
            c.insert_one(user).await.unwrap();
        }

        pub async fn get_user_from_db(db: Database, user_id: &str) -> UserDocument {
            db.collection("users")
                .find_one(doc! {"_id": user_id})
                .await
                .unwrap()
                .unwrap()
        }
    }
}
