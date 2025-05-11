pub mod permissions {
    #[derive(Debug, PartialEq, Clone)]
    pub enum Permission {
        BanUser,
        UnbanUser,
        CreatePost,
        DeletePost,
        UpdatePost,
        DeleteUser,
        CreateAccount,
        AwardBadge,
        RevokeBadge,
        MakeModerator,
        ViewUser,
        ListUsers,
        MakeRegular,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum UserPermission {
        BanUser,
        UnbanUser,
        CreateAccount,
        AwardBadge,
        RevokeBadge,
        MakeModerator,
        ViewUser,
        ListUsers,
        MakeRegular,
    }

    impl From<UserPermission> for Permission {
        fn from(p: UserPermission) -> Self {
            match p {
                UserPermission::BanUser => Permission::BanUser,
                UserPermission::UnbanUser => Permission::UnbanUser,
                UserPermission::AwardBadge => Permission::AwardBadge,
                UserPermission::RevokeBadge => Permission::RevokeBadge,
                UserPermission::MakeModerator => Permission::MakeModerator,
                UserPermission::ViewUser => Permission::ViewUser,
                UserPermission::ListUsers => Permission::ListUsers,
                UserPermission::MakeRegular => Permission::MakeRegular,
                UserPermission::CreateAccount => Permission::CreateAccount,
            }
        }
    }
}
pub mod roles {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
    pub enum UserRole {
        Admin,
        Regular,
        Moderator,
        Guest,
    }
}
