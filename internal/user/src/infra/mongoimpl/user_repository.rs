use mongodb::{Collection, Database, bson::doc};

use crate::domain::{
    errors::UserDomainError,
    result::UserDomainResult,
    user::{EmailStatus, User},
};
use shared::db_transactions::{DBTransaction, RepoDB};

use super::user_document::UserDocument;

pub struct MongoUserRepository {
    collection: Collection<UserDocument>,
    db: Database,
}

impl MongoUserRepository {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection("users"),
            db,
        }
    }
    pub fn get_repo_db(&self) -> RepoDB {
        RepoDB::MongoDb(self.db.clone())
    }
    async fn find_and_update_user<F: FnOnce(&mut User) + Send>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
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
    pub async fn create_account(&self, user: User) -> UserDomainResult<()> {
        let user: UserDocument = user.into();
        self.collection.insert_one(user).await?;
        Ok(())
    }

    pub async fn make_moderator<F: FnOnce(&mut User) + Send>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    pub async fn change_username<F: FnOnce(&mut User) + Send>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    pub async fn award_badge<F: FnOnce(&mut User) + Send>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    pub async fn revoke_badge<F: FnOnce(&mut User) + Send>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    pub async fn ban_user<F: FnOnce(&mut User) + Send>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    pub async fn unban_user<F: FnOnce(&mut User) + Send>(
        &self,
        user_id: &str,
        update_fn: F,
    ) -> UserDomainResult<()> {
        self.find_and_update_user(user_id, update_fn).await
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> UserDomainResult<Option<User>> {
        let user = self
            .collection
            .find_one(doc! {"_id": user_id})
            .await?
            .map(|doc| doc.into());
        Ok(user)
    }
    pub async fn get_user_by_username_or_email(
        &self,
        username: &str,
        email: &str,
    ) -> UserDomainResult<Option<User>> {
        let user = self
            .collection
            .find_one(doc! {"$or": [{"username": username}, {"email": email}]})
            .await?
            .map(|doc| doc.into());
        Ok(user)
    }

    pub async fn user_exists(
        &self,
        username: &str,
        email: &str,
        email_status: Option<EmailStatus>,
    ) -> UserDomainResult<bool> {
        let mut filter = doc! {
            "$or": [
                {"email": email},
                {"username": username}
            ]
        };
        if let Some(status) = email_status {
            filter.insert("email_status", status.to_string());
        }
        // Check if the user exists in the database
        let user = self.collection.find_one(filter).await?;
        Ok(user.is_some())
    }
    pub async fn upsert_user<'a>(
        &self,
        user: User,
        tx: Option<DBTransaction<'a>>,
    ) -> UserDomainResult<()> {
        let filter = doc! {
            "$or": [
                {"email": user.email()},
                {"username": user.username()}
            ]
        };
        let user: UserDocument = user.into();
        let fr = self
            .collection
            .find_one_and_replace(filter, user)
            .upsert(true);
        if let Some(tx) = tx {
            match tx {
                DBTransaction::MongoDb(s) => {
                    fr.session(s).await?;
                }
                _ => {
                    return Err(UserDomainError::InvalidTransaction);
                }
            }
        } else {
            fr.await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user::BanType;

    use super::*;
    use shared::test_utils;
    use utils::{assert_user_in_db_equals, get_user_from_db, insert_user};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_account() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);
        let user_repo = MongoUserRepository::new(db.clone());
        user_repo.create_account(user.clone()).await.unwrap();
        assert_user_in_db_equals(user, db).await;
    }

    #[tokio::test]
    async fn test_user_exists() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));

        let user = User::new_test_user(None);
        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        let res = user_repo
            .user_exists(user.username(), user.email(), Some(EmailStatus::Verified))
            .await
            .unwrap();
        assert_eq!(true, res);
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
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
    async fn test_make_moderator() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let mut user = User::new_test_user(None);

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .make_moderator(user.id(), |u| {
                u.make_moderator();
            })
            .await
            .unwrap();
        user.make_moderator();
        assert_user_in_db_equals(user, db).await;
    }

    #[tokio::test]
    async fn test_ban_user() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .ban_user(user.id(), |u| {
                u.ban("abuse".into(), BanType::Indefinite);
            })
            .await
            .unwrap();
        let user_from_db = get_user_from_db(db.clone(), user.id()).await;
        assert_eq!(true, user_from_db.ban_status.unwrap().is_banned);
    }

    #[tokio::test]
    async fn test_unban_user() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .unban_user(user.id(), |u| {
                u.unban();
            })
            .await
            .unwrap();
        let user_from_db = get_user_from_db(db.clone(), user.id()).await;
        assert_eq!(None, user_from_db.ban_status);
    }

    #[tokio::test]
    async fn test_award_badge() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .award_badge(user.id(), |u| {
                u.award_badge("Helpful".into());
            })
            .await
            .unwrap();
        let user_from_db = get_user_from_db(db.clone(), user.id()).await;
        assert_eq!(1, user_from_db.badges.len());
    }

    #[tokio::test]
    async fn test_revoke_badge() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let mut user = User::new_test_user(None);

        let badge = "Helpful".to_string();
        user.award_badge(badge.clone());

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .revoke_badge(user.id(), |u| {
                u.revoke_badge(badge);
            })
            .await
            .unwrap();
        let user_from_db = get_user_from_db(db.clone(), user.id()).await;
        assert_eq!(0, user_from_db.badges.len());
    }

    #[tokio::test]
    async fn test_change_username() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);

        let username = "new_username".to_string();

        let expected_username = username.clone();

        insert_user(db.clone(), user.clone()).await;

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo
            .change_username(user.id(), |u| {
                u.change_username(username);
            })
            .await
            .unwrap();
        let user_from_db = get_user_from_db(db.clone(), user.id()).await;
        assert_eq!(expected_username.to_string(), user_from_db.username);
    }

    #[tokio::test]
    async fn test_upsert_user() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));
        let user = User::new_test_user(None);

        let user_repo = MongoUserRepository::new(db.clone());
        user_repo.upsert_user(user.clone(), None).await.unwrap();
        assert_user_in_db_equals(user.clone(), db.clone()).await;

        // Update the user
        let mut updated_user = user.clone();
        updated_user.change_username("updated_username".to_string());
        user_repo
            .upsert_user(updated_user.clone(), None)
            .await
            .unwrap();
        assert_user_in_db_equals(updated_user, db).await;
    }

    #[tokio::test]
    async fn test_upsert_user_in_transaction() {
        let client = test_utils::setup_test_mongo().await;
        let db = client.database(&format!("test_db-{}", Uuid::new_v4().to_string()));

        let user_repo = MongoUserRepository::new(db.clone());
        let repo_db = user_repo.get_repo_db();
        match repo_db {
            RepoDB::MongoDb(db) => {
                let user = User::new_test_user(None);
                let mut session = db.client().start_session().await.unwrap();
                session.start_transaction().await.unwrap();
                user_repo
                    .upsert_user(user.clone(), Some(DBTransaction::MongoDb(&mut session)))
                    .await
                    .unwrap();

                // Assert that the user is not in the database yet before committing the transaction
                let res = user_repo
                    .collection
                    .find_one(doc! {"_id": user.id()})
                    .await
                    .unwrap();
                assert!(res.is_none());

                // Commit the transaction
                session.commit_transaction().await.unwrap();

                // Assert that the user is in the database after committing the transaction
                assert_user_in_db_equals(user.clone(), db).await;
            }
            _ => panic!("Expected MongoDB transaction"),
        }
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
