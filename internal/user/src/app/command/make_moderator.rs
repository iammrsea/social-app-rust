use std::sync::Arc;

use async_trait::async_trait;

use shared::{
    auth::AuthenticatedUser,
    command_handler::CommandHanlder,
    guards::{
        Guards,
        rbac::{permissions::Permission, roles::UserRole},
    },
    types::AppResult,
};

use crate::domain::user_repository::UserRepository;

pub struct MakeModerator {
    pub user_id: String,
}

pub struct MakeModeratorHandler {
    repo: Arc<dyn UserRepository>,
    guard: Arc<dyn Guards>,
}

impl MakeModeratorHandler {
    pub fn new(repo: Arc<dyn UserRepository>, guard: Arc<dyn Guards>) -> Self {
        Self { repo, guard }
    }
}

#[async_trait]
impl CommandHanlder<MakeModerator> for MakeModeratorHandler {
    async fn handle(&self, cmd: MakeModerator) -> AppResult<()> {
        let auth_user = AuthenticatedUser::new(UserRole::Admin); // TODO: Get auth user from context
        self.guard
            .authorize(&auth_user.role, &Permission::MakeModerator)?;
        self.repo
            .make_moderator(
                &cmd.user_id,
                Box::new(|user| {
                    user.make_moderator();
                }),
            )
            .await?;
        Ok(())
    }
}
