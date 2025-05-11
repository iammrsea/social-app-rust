use std::collections::HashMap;

use shared::guards::permissions::Permission;
use shared::guards::permissions::Permission::{
    BanUser, CreateAccount, ListUsers, UnbanUser, ViewUser,
};
use shared::guards::roles::UserRole;
use shared::guards::roles::UserRole::{Admin, Guest, Moderator, Regular};

#[derive(Debug, Clone)]
pub struct Policy {
    rules: HashMap<UserRole, Vec<Permission>>,
}

impl Policy {
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        rules.insert(Admin, vec![ViewUser]);
        rules.insert(Regular, vec![ViewUser]);
        rules.insert(Moderator, vec![ViewUser, ListUsers, BanUser, UnbanUser]);
        rules.insert(Guest, vec![CreateAccount]);
        Self { rules }
    }
    pub fn is_allowed(&self, role: &UserRole, perm: &Permission) -> bool {
        if role == &Admin {
            return true;
        }
        self.rules
            .get(role)
            .map_or(false, |perms| perms.contains(perm))
    }
}
