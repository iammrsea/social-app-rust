use shared::guards::permissions::Permission;
use shared::guards::roles::UserRole;
use shared::{errors::user::UserDomainError, types::AppResult};

pub struct RbacEngine;

impl RbacEngine {
    pub fn new() -> Self {
        Self
    }
    pub fn authorize(&self, role: &UserRole, perm: &Permission) -> AppResult<()> {
        let p = super::policy::Policy::new();
        if !p.is_allowed(role, perm) {
            return Err(UserDomainError::Unauthorized.into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::guards::permissions::Permission::{
        AwardBadge, BanUser, CreateAccount, DeleteUser, ListUsers, MakeModerator, MakeRegular,
        RevokeBadge, UnbanUser, ViewUser,
    };
    use shared::guards::roles::UserRole::{Admin, Guest, Moderator, Regular};

    #[test]
    fn admin_has_ban_permission() {
        let r = RbacEngine::new().authorize(&Admin, &BanUser);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn moderator_has_ban_permission() {
        let r = RbacEngine::new().authorize(&Moderator, &BanUser);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn regular_user_has_no_ban_permission() {
        let r = RbacEngine::new().authorize(&Regular, &BanUser);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn guest_has_no_ban_permission() {
        let r = RbacEngine::new().authorize(&Guest, &BanUser);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn admin_has_unban_permission() {
        let r = RbacEngine::new().authorize(&Admin, &UnbanUser);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn moderator_has_unban_permission() {
        let r = RbacEngine::new().authorize(&Moderator, &UnbanUser);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn regular_user_has_no_unban_permission() {
        let r = RbacEngine::new().authorize(&Regular, &UnbanUser);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn guest_has_no_unban_permission() {
        let r = RbacEngine::new().authorize(&Guest, &UnbanUser);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn admin_has_view_user_permission() {
        let r = RbacEngine::new().authorize(&Admin, &ViewUser);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn moderator_has_view_user_permission() {
        let r = RbacEngine::new().authorize(&Moderator, &ViewUser);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn regular_user_has_view_user_permission() {
        let r = RbacEngine::new().authorize(&Regular, &ViewUser);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn guest_has_no_view_user_permission() {
        let r = RbacEngine::new().authorize(&Guest, &ViewUser);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn admin_has_delete_user_permission() {
        let r = RbacEngine::new().authorize(&Admin, &DeleteUser);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn moderator_has_no_delete_user_permission() {
        let r = RbacEngine::new().authorize(&Moderator, &DeleteUser);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn regular_user_has_no_delete_user_permission() {
        let r = RbacEngine::new().authorize(&Regular, &DeleteUser);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn guest_has_no_delete_user_permission() {
        let r = RbacEngine::new().authorize(&Guest, &DeleteUser);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn admin_has_award_badge_permission() {
        let r = RbacEngine::new().authorize(&Admin, &AwardBadge);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn moderator_has_no_award_badge_permission() {
        let r = RbacEngine::new().authorize(&Moderator, &AwardBadge);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn regular_user_has_no_award_badge_permission() {
        let r = RbacEngine::new().authorize(&Regular, &AwardBadge);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn guest_has_no_award_badge_permission() {
        let r = RbacEngine::new().authorize(&Guest, &AwardBadge);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn admin_has_revoke_badge_permission() {
        let r = RbacEngine::new().authorize(&Admin, &RevokeBadge);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn moderator_has_no_revoke_badge_permission() {
        let r = RbacEngine::new().authorize(&Moderator, &RevokeBadge);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn regular_user_has_no_revoke_badge_permission() {
        let r = RbacEngine::new().authorize(&Regular, &RevokeBadge);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn guest_has_no_revoke_badge_permission() {
        let r = RbacEngine::new().authorize(&Guest, &RevokeBadge);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn admin_has_make_moderator_permission() {
        let r = RbacEngine::new().authorize(&Admin, &MakeModerator);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn moderator_has_no_make_moderator_permission() {
        let r = RbacEngine::new().authorize(&Moderator, &MakeModerator);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn regular_user_has_no_make_moderator_permission() {
        let r = RbacEngine::new().authorize(&Regular, &MakeModerator);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn guest_has_no_make_moderator_permission() {
        let r = RbacEngine::new().authorize(&Guest, &MakeModerator);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn admin_has_make_regular_permission() {
        let r = RbacEngine::new().authorize(&Admin, &MakeRegular);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn moderator_has_no_make_regular_permission() {
        let r = RbacEngine::new().authorize(&Moderator, &MakeRegular);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn regular_user_has_no_make_regular_permission() {
        let r = RbacEngine::new().authorize(&Regular, &MakeRegular);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn guest_has_no_make_regular_permission() {
        let r = RbacEngine::new().authorize(&Guest, &MakeRegular);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn admin_has_list_users_permission() {
        let r = RbacEngine::new().authorize(&Admin, &ListUsers);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn moderator_has_list_users_permission() {
        let r = RbacEngine::new().authorize(&Moderator, &ListUsers);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn regular_user_has_no_list_users_permission() {
        let r = RbacEngine::new().authorize(&Regular, &ListUsers);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn guest_has_no_list_users_permission() {
        let r = RbacEngine::new().authorize(&Guest, &ListUsers);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn admin_has_create_account_permission() {
        let r = RbacEngine::new().authorize(&Admin, &CreateAccount);
        assert_eq!(r.is_ok(), true);
    }

    #[test]
    fn moderator_has_no_create_account_permission() {
        let r = RbacEngine::new().authorize(&Moderator, &CreateAccount);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn regular_user_has_no_create_account_permission() {
        let r = RbacEngine::new().authorize(&Regular, &CreateAccount);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn guest_has_create_account_permission() {
        let r = RbacEngine::new().authorize(&Guest, &CreateAccount);
        assert_eq!(r.is_ok(), true);
    }
}
