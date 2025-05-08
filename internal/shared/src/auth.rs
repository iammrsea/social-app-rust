use crate::guards::rbac::roles::UserRole;

pub struct AuthenticatedUser {
    pub email: String,
    pub id: String,
    pub role: UserRole,
}

impl AuthenticatedUser {
    pub fn new(role: UserRole) -> Self {
        Self {
            id: "user_id12345".into(),
            email: "johndoe@example.com".into(),
            role,
        }
    }
}
