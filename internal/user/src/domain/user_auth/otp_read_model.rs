use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct OtpReadModel {
    pub value: String,
    pub email: String,
}
