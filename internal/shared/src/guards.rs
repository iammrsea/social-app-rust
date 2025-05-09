/// Provides implementation for attribute-based access control
pub mod abac;
/// Provides implementation for role-based access control
pub mod rbac;

pub trait Guards: rbac::RbacGuard + abac::AbacGuard + Send + Sync {}

pub struct GuardsImpl;

impl GuardsImpl {
    pub fn new() -> Self {
        Self {}
    }
}
impl Guards for GuardsImpl {}
