use crate::{
    auth::AuthenticatedUser,
    types::{AppResult, non_empty_string::NonEmptyString},
};

use super::Guards;

pub trait AbacGuard {
    fn can_change_username(
        &self,
        _user_id: &NonEmptyString,
        auth_user: &AuthenticatedUser,
    ) -> AppResult<()>;
}

impl<T> AbacGuard for T
where
    T: Guards,
{
    fn can_change_username(
        &self,
        user_id: &NonEmptyString,
        auth_user: &AuthenticatedUser,
    ) -> AppResult<()> {
        user::can_change_username(user_id, auth_user)
    }
}

pub mod user {
    use crate::auth::AuthenticatedUser;
    use crate::errors::user::UserDomainError;
    use crate::guards::rbac::roles::UserRole::{Admin, Guest, Moderator, Regular};
    use crate::types::AppResult;

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
