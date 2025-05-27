use std::sync::Arc;

use crate::infra::repository::otp_repository::OtpRepository;

use crate::guards::UserGuards;

use crate::infra::repository::{
    user_read_model_repository::UserReadModelRepository, user_repository::UserRepository,
};

use super::{
    command::{
        award_badge::AwardBadgeHandler, ban_user::BanUserHandler,
        change_username::ChangeUsernameHandler, make_moderator::MakeModeratorHandler,
        revoke_badge::RevokeBadgeHandler, sign_in::SignInHandler, sign_up::SignUpHandler,
        unban_user::UnbanUserHandler, verify_email_with_otp::VerifyEmailWithOtpHandler,
        verify_otp::VerifyOtpHandler,
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
        user_repo: Arc<UserRepository>,
        user_read_repo: Arc<UserReadModelRepository>,
        guard: Arc<dyn UserGuards>,
        otp_repo: Arc<OtpRepository>,
    ) -> Self {
        Self {
            command_handler: CommandHandler {
                sign_up: SignUpHandler::new(user_repo.clone(), otp_repo.clone()),
                award_badge: AwardBadgeHandler::new(user_repo.clone(), guard.clone()),
                revoke_badge: RevokeBadgeHandler::new(user_repo.clone(), guard.clone()),
                make_moderator: MakeModeratorHandler::new(user_repo.clone(), guard.clone()),
                ban_user: BanUserHandler::new(user_repo.clone(), guard.clone()),
                unban_user: UnbanUserHandler::new(user_repo.clone(), guard.clone()),
                change_username: ChangeUsernameHandler::new(user_repo.clone(), guard.clone()),
                verify_otp: VerifyOtpHandler::new(user_repo.clone(), otp_repo.clone()),
                verify_email_with_opt: VerifyEmailWithOtpHandler::new(
                    user_repo.clone(),
                    otp_repo.clone(),
                ),
                sign_in: SignInHandler::new(user_repo.clone(), otp_repo.clone()),
            },
            query_handler: QueryHandler {
                get_user_by_id: GetUserByIdHander::new(user_read_repo.clone(), guard.clone()),
                get_user_by_email: GetUserByEmailHander::new(user_read_repo.clone(), guard.clone()),
                get_users: GetUsersHandler::new(user_read_repo.clone(), guard.clone()),
            },
        }
    }
}

pub struct CommandHandler {
    pub sign_up: SignUpHandler,
    pub award_badge: AwardBadgeHandler,
    pub revoke_badge: RevokeBadgeHandler,
    pub make_moderator: MakeModeratorHandler,
    pub ban_user: BanUserHandler,
    pub unban_user: UnbanUserHandler,
    pub change_username: ChangeUsernameHandler,
    pub verify_otp: VerifyOtpHandler,
    pub verify_email_with_opt: VerifyEmailWithOtpHandler,
    pub sign_in: SignInHandler,
}

pub struct QueryHandler {
    pub get_user_by_id: GetUserByIdHander,
    pub get_user_by_email: GetUserByEmailHander,
    pub get_users: GetUsersHandler,
}
