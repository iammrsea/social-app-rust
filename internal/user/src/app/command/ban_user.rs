use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::AuthenticatedUser,
    command_handler::CommandHanlder,
    guards::{
        Guards,
        rbac::{permissions::Permission, roles::UserRole},
    },
    types::{AppResult, non_empty_string::NonEmptyString},
};

use crate::domain::{user::BanType, user_repository::UserRepository};

pub struct BanUser {
    pub user_id: String,
    pub reason: NonEmptyString,
    pub ban_type: BanType,
}

pub struct BanUserHandler {
    repo: Arc<dyn UserRepository>,
    guard: Arc<dyn Guards>,
}

impl BanUserHandler {
    pub fn new(repo: Arc<dyn UserRepository>, guard: Arc<dyn Guards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<BanUser> for BanUserHandler {
    async fn handle(&self, cmd: BanUser) -> AppResult<()> {
        let auth_user = AuthenticatedUser::new(UserRole::Admin); // TODO: Get auth user from context
        self.guard
            .authorize(&auth_user.role, &Permission::BanUser)?;
        self.repo
            .ban_user(
                &cmd.user_id,
                Box::new(|user| {
                    user.ban(cmd.reason, cmd.ban_type);
                }),
            )
            .await?;
        Ok(())
    }
}
