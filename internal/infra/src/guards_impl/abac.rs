use ::user::domain::result::UserDomainResult;
use shared::auth::AuthUser;

pub struct AbacEngine;

impl AbacEngine {
    pub fn new() -> Self {
        Self
    }
    pub fn can_change_username(&self, user_id: &str, auth_user: &AuthUser) -> UserDomainResult<()> {
        user::can_change_username(user_id, auth_user)
    }
}

pub mod user {
    use shared::auth::AuthUser;
    use shared::guards::roles::UserRole::{Admin, Guest, Moderator, Regular};
    use user::domain::{errors::UserDomainError, result::UserDomainResult};

    pub fn can_change_username(user_id: &str, auth_user: &AuthUser) -> UserDomainResult<()> {
        match auth_user.0.role {
            Admin => Ok(()),
            Regular | Moderator => {
                if user_id == auth_user.0.id {
                    return Ok(());
                }
                return Err(UserDomainError::Unauthorized);
            }
            Guest => Err(UserDomainError::Unauthorized),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn regular_user_can_change_their_username() {
            let auth_user = AuthUser::new_test_auth_user(Regular);
            let user_id = auth_user.0.id.clone();
            let can_change = can_change_username(user_id.as_str(), &auth_user);
            assert_eq!(true, can_change.is_ok());
        }

        #[test]
        fn moderator_can_change_their_username() {
            let auth_user = AuthUser::new_test_auth_user(Moderator);
            let user_id = auth_user.0.id.clone();
            let can_change = can_change_username(user_id.as_str(), &auth_user);
            assert_eq!(true, can_change.is_ok());
        }

        #[test]
        fn admin_can_change_username() {
            let auth_user = AuthUser::new_test_auth_user(Admin);
            let can_change = can_change_username("user-id", &auth_user);
            assert_eq!(true, can_change.is_ok());
        }

        #[test]
        fn regular_user_cannot_change_username_by_proxy() {
            let auth_user = AuthUser::new_test_auth_user(Regular);
            let can_change = can_change_username("user-id", &auth_user);
            assert_eq!(true, can_change.is_err());
        }

        #[test]
        fn moderator_cannot_change_username_by_proxy() {
            let auth_user = AuthUser::new_test_auth_user(Moderator);
            let can_change = can_change_username("user-id", &auth_user);
            assert_eq!(true, can_change.is_err());
        }

        #[test]
        fn guest_cannot_change_username() {
            let auth_user = AuthUser::new_test_auth_user(Guest);
            let user_id = auth_user.0.id.clone();
            let can_change = can_change_username(user_id.as_str(), &auth_user);
            assert_eq!(true, can_change.is_err());
        }
    }
}
