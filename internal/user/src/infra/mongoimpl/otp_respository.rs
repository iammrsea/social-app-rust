use crate::domain::user_auth::{errors::UserAuthError, otp::OtpEntry, result::UserAuthResult};
use mongodb::{Collection, Database, bson::doc};
use serde::{Deserialize, Serialize};
use shared::db_transactions::DBTransaction;

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
        OtpEntry::new(
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
    pub async fn upsert_otp<'a>(
        &self,
        otp: OtpEntry,
        tx: Option<DBTransaction<'a>>,
    ) -> UserAuthResult<()> {
        let doc: OtpDocument = otp.into();
        let fr = self
            .collection
            .find_one_and_replace(doc! {"email": &doc.email }, &doc)
            .upsert(true);

        if let Some(tx) = tx {
            match tx {
                DBTransaction::MongoDb(session) => {
                    fr.session(session).await?;
                }
                _ => {
                    return Err(UserAuthError::InvalidTransaction);
                }
            }
        } else {
            fr.await?;
        }

        Ok(())
    }

    pub async fn update_opt(&self, otp: OtpEntry) -> UserAuthResult<()> {
        let document: OtpDocument = otp.into();
        self.collection
            .replace_one(doc! { "email": &document.email }, document)
            .await?;
        Ok(())
    }

    pub async fn get_otp_by_user_email(&self, email: &str) -> UserAuthResult<Option<OtpEntry>> {
        let otp = self
            .collection
            .find_one(doc! {"email": email})
            .await?
            .map(|doc| doc.into());
        Ok(otp)
    }

    pub async fn delete_otp(&self, email: &str) -> UserAuthResult<()> {
        let filter = doc! { "email": email };
        self.collection.delete_one(filter).await?;
        Ok(())
    }
}
