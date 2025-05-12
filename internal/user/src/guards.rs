use shared::{
    auth::AuthUser,
    guards::{permissions::UserPermission, roles::UserRole},
    types::AppResult,
};

#[cfg_attr(test, mockall::automock)]
pub trait UserGuards: Send + Sync {
    fn authorize(&self, role: &UserRole, perm: &UserPermission) -> AppResult<()>;
    fn can_change_username(&self, user_id: &str, auth_user: &AuthUser) -> AppResult<()>;
}
