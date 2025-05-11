use shared::{auth::AuthenticatedUser, types::AppResult};

pub struct AbacEngine;

impl AbacEngine {
    pub fn new() -> Self {
        Self
    }
    pub fn can_change_username(
        &self,
        user_id: &str,
        auth_user: &AuthenticatedUser,
    ) -> AppResult<()> {
        user::can_change_username(user_id, auth_user)
    }
}

pub mod user {
    use shared::auth::AuthenticatedUser;
    use shared::errors::user::UserDomainError;
    use shared::guards::roles::UserRole::{Admin, Guest, Moderator, Regular};
    use shared::types::AppResult;

    pub fn can_change_username(user_id: &str, auth_user: &AuthenticatedUser) -> AppResult<()> {
        match auth_user.role {
            Admin => Ok(()),
            Regular | Moderator => {
                if user_id == auth_user.id {
                    return Ok(());
                }
                return Err(UserDomainError::Unauthorized.into());
            }
            Guest => Err(UserDomainError::Unauthorized.into()),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn regular_user_can_change_their_username() {
            let auth_user = AuthenticatedUser::new(Regular);
            let user_id = auth_user.id.clone();
            let can_change = can_change_username(user_id.as_str(), &auth_user);
            assert_eq!(true, can_change.is_ok());
        }

        #[test]
        fn moderator_can_change_their_username() {
            let auth_user = AuthenticatedUser::new(Moderator);
            let user_id = auth_user.id.clone();
            let can_change = can_change_username(user_id.as_str(), &auth_user);
            assert_eq!(true, can_change.is_ok());
        }

        #[test]
        fn admin_can_change_username() {
            let auth_user = AuthenticatedUser::new(Admin);
            let can_change = can_change_username("user-id", &auth_user);
            assert_eq!(true, can_change.is_ok());
        }

        #[test]
        fn regular_user_cannot_change_username_by_proxy() {
            let auth_user = AuthenticatedUser::new(Regular);
            let can_change = can_change_username("user-id", &auth_user);
            assert_eq!(true, can_change.is_err());
        }

        #[test]
        fn moderator_cannot_change_username_by_proxy() {
            let auth_user = AuthenticatedUser::new(Moderator);
            let can_change = can_change_username("user-id", &auth_user);
            assert_eq!(true, can_change.is_err());
        }

        #[test]
        fn guest_cannot_change_username() {
            let auth_user = AuthenticatedUser::new(Guest);
            let user_id = auth_user.id.clone();
            let can_change = can_change_username(user_id.as_str(), &auth_user);
            assert_eq!(true, can_change.is_err());
        }
    }
}
