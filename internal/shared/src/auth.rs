use crate::guards::roles::UserRole;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub user: Option<AuthUser>,
}

impl AppContext {
    pub fn new() -> Self {
        Self { user: None }
    }
    pub fn with_user(mut self, user: AuthUser) -> Self {
        self.user = Some(user);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthUser {
    pub email: String,
    pub id: String,
    pub role: UserRole,
}

impl AuthUser {
    pub fn new(role: UserRole, id: String, email: String) -> Self {
        Self { id, email, role }
    }
    pub fn guest() -> Self {
        Self {
            email: "".into(),
            id: "".into(),
            role: UserRole::Guest,
        }
    }
    pub fn new_test_auth_user(role: UserRole) -> Self {
        Self {
            id: "user_id".into(),
            email: "johndoe@example.com".into(),
            role,
        }
    }
}

pub fn get_auth_user_from_ctx<'ctx>(ctx: &'ctx AppContext) -> &'ctx AuthUser {
    ctx.user.as_ref().unwrap()
}
