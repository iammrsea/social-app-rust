use std::sync::Arc;

use crate::guards::UserGuards;

use crate::domain::{
    user_read_model_repository::UserReadModelRepository, user_repository::UserRepository,
};

use super::{
    command::{
        award_badge::AwardBadgeHandler, ban_user::BanUserHandler,
        change_username::ChangeUsernameHandler, create_account::CreateAccountHandler,
        make_moderator::MakeModeratorHandler, revoke_badge::RevokeBadgeHandler,
        unban_user::UnbanUserHandler,
    },
    query::{
        user_by_email::GetUserByEmailHander, user_by_id::GetUserByIdHander, users::GetUsersHandler,
    },
};

pub struct UserService {
    pub command_handler: CommandHandler,
    pub query_handler: QueryHandler,
}

impl UserService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        user_read_repo: Arc<dyn UserReadModelRepository>,
        guard: Arc<dyn UserGuards>,
    ) -> Self {
        Self {
            command_handler: CommandHandler {
                create_account: CreateAccountHandler::new(user_repo.clone(), guard.clone()),
                award_badge: AwardBadgeHandler::new(user_repo.clone(), guard.clone()),
                revoke_badge: RevokeBadgeHandler::new(user_repo.clone(), guard.clone()),
                make_moderator: MakeModeratorHandler::new(user_repo.clone(), guard.clone()),
                ban_user: BanUserHandler::new(user_repo.clone(), guard.clone()),
                unban_user: UnbanUserHandler::new(user_repo.clone(), guard.clone()),
                change_username: ChangeUsernameHandler::new(user_repo.clone(), guard.clone()),
            },
            query_handler: QueryHandler {
                get_user_by_id: GetUserByIdHander::new(user_read_repo.clone(), guard.clone()),
                get_user_email: GetUserByEmailHander::new(user_read_repo.clone(), guard.clone()),
                get_users: GetUsersHandler::new(user_read_repo.clone(), guard.clone()),
            },
        }
    }
}

pub struct CommandHandler {
    pub create_account: CreateAccountHandler,
    pub award_badge: AwardBadgeHandler,
    pub revoke_badge: RevokeBadgeHandler,
    pub make_moderator: MakeModeratorHandler,
    pub ban_user: BanUserHandler,
    pub unban_user: UnbanUserHandler,
    pub change_username: ChangeUsernameHandler,
}

pub struct QueryHandler {
    pub get_user_by_id: GetUserByIdHander,
    pub get_user_email: GetUserByEmailHander,
    pub get_users: GetUsersHandler,
}
