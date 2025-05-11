use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::AuthenticatedUser,
    command_handler::CommandHanlder,
    guards::{permissions::UserPermission, roles::UserRole},
    types::{AppResult, non_empty_string::NonEmptyString},
};

use crate::domain::user_repository::UserRepository;
use crate::guards::UserGuards;

pub struct AwardBadge {
    pub user_id: String,
    pub badge: NonEmptyString,
}

pub struct AwardBadgeHandler {
    repo: Arc<dyn UserRepository>,
    guard: Arc<dyn UserGuards>,
}

impl AwardBadgeHandler {
    pub fn new(repo: Arc<dyn UserRepository>, guard: Arc<dyn UserGuards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<AwardBadge> for AwardBadgeHandler {
    async fn handle(&self, cmd: AwardBadge) -> AppResult<()> {
        let auth_user = AuthenticatedUser::new(UserRole::Admin); // TODO: Get auth user from context
        self.guard
            .authorize(&auth_user.role, &UserPermission::AwardBadge)?;
        self.repo
            .revoke_badge(
                &cmd.user_id,
                Box::new(|user| {
                    user.revoke_badge(cmd.badge);
                }),
            )
            .await?;
        Ok(())
    }
}
