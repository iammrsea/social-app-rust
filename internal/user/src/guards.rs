use crate::domain::result::UserDomainResult;
use shared::{
    auth::AuthUser,
    guards::{permissions::UserPermission, roles::UserRole},
};

#[cfg_attr(test, mockall::automock)]
pub trait UserGuards: Send + Sync {
    fn authorize(&self, role: &UserRole, perm: &UserPermission) -> UserDomainResult<()>;
    fn can_change_username(&self, user_id: &str, auth_user: &AuthUser) -> UserDomainResult<()>;
}
