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
