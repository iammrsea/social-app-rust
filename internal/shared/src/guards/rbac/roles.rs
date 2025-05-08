use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum UserRole {
    Admin,
    Regular,
    Moderator,
    Guest,
}
