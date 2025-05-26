use super::{OtpEntry, otp_respository::OtpRepository};
use crate::{errors::AuthError, result::AuthResult};
use mongodb::{Collection, Database, bson::doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtpDocument {
    pub otp_hash: String,
    pub expires_at: i64,
    pub used: bool,
    pub attempts: u32,
    pub email: String,
}

impl From<OtpDocument> for OtpEntry {
    fn from(doc: OtpDocument) -> Self {
        OtpEntry::new_with_all_fields(
            doc.email,
            doc.used,
            doc.attempts,
            doc.otp_hash,
            doc.expires_at,
        )
    }
}
impl Into<OtpDocument> for OtpEntry {
    fn into(self) -> OtpDocument {
        OtpDocument {
            otp_hash: self.otp_hash().to_string(),
            expires_at: self.expires_at().to_owned(),
            used: self.used().to_owned(),
            attempts: self.attempts().to_owned(),
            email: self.email().to_string(),
        }
    }
}

pub struct MongoOtpRepository {
    collection: Collection<OtpDocument>,
}

impl MongoOtpRepository {
    pub fn new(db: Database) -> Self {
        let collection = db.collection::<OtpDocument>("otps");
        Self { collection }
    }
    pub async fn upsert_otp(&self, otp: OtpEntry) -> AuthResult<()> {
        let doc: OtpDocument = otp.into();
        self.collection
            .find_one_and_replace(doc! {"email": &doc.email }, &doc)
            .upsert(true)
            .await?;
        Ok(())
    }

    pub async fn update_opt(&self, otp: OtpEntry) -> AuthResult<()> {
        let document: OtpDocument = otp.into();
        self.collection
            .replace_one(doc! { "email": &document.email }, document)
            .await?;
        Ok(())
    }

    pub async fn get_otp_by_user_email(&self, email: &str) -> AuthResult<OtpEntry> {
        let filter = doc! { "email": email };
        if let Some(opt) = self.collection.find_one(filter).await? {
            Ok(opt.into())
        } else {
            Err(AuthError::OtpNotFound)
        }
    }

    pub async fn delete_otp(&self, email: &str) -> AuthResult<()> {
        let filter = doc! { "email": email };
        self.collection.delete_one(filter).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl OtpRepository for MongoOtpRepository {
    async fn upsert_otp(&self, otp: OtpEntry) -> AuthResult<()> {
        let doc: OtpDocument = otp.into();
        self.collection
            .find_one_and_replace(doc! {"email": &doc.email }, &doc)
            .upsert(true)
            .await?;
        Ok(())
    }

    async fn update_opt(&self, otp: OtpEntry) -> AuthResult<()> {
        let document: OtpDocument = otp.into();
        self.collection
            .replace_one(doc! { "email": &document.email }, document)
            .await?;
        Ok(())
    }

    async fn get_otp_by_user_email(&self, email: &str) -> AuthResult<OtpEntry> {
        let filter = doc! { "email": email };
        if let Some(opt) = self.collection.find_one(filter).await? {
            Ok(opt.into())
        } else {
            Err(AuthError::OtpNotFound)
        }
    }

    async fn delete_otp(&self, email: &str) -> AuthResult<()> {
        let filter = doc! { "email": email };
        self.collection.delete_one(filter).await?;
        Ok(())
    }
}
