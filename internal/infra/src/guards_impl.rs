use abac::AbacEngine;
use rbac::RbacEngine;
use shared::{
    auth::AuthUser,
    guards::{
        permissions::{Permission, UserPermission},
        roles::UserRole,
    },
};
use user::domain::{errors::UserDomainError, result::UserDomainResult};

mod abac;
mod policy;
mod rbac;

pub struct GuardsImpl {
    rbac: RbacEngine,
    abac: AbacEngine,
}

impl GuardsImpl {
    pub fn new() -> Self {
        Self {
            rbac: RbacEngine::new(),
            abac: AbacEngine::new(),
        }
    }
}

impl user::guards::UserGuards for GuardsImpl {
    fn can_change_username(&self, user_id: &str, auth_user: &AuthUser) -> UserDomainResult<()> {
        self.abac.can_change_username(user_id, auth_user)
    }
    fn authorize(&self, role: &UserRole, perm: &UserPermission) -> UserDomainResult<()> {
        let internal = Permission::from(perm.clone());
        match self.rbac.authorize(role, &internal) {
            Ok(_) => Ok(()),
            Err(..) => Err(UserDomainError::Unauthorized),
        }
    }
}
