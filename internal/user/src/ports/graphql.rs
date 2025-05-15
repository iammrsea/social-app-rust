// use async_graphql::{Context, Object};

#[derive(Default, Debug)]
pub struct UserQuery;

// #[Object]
// impl UserQuery {
//     async fn user(&self, ctx: &Context<'_>, id: String) -> Option<User> {
//         let user = ctx.data::<UserRepository>().unwrap().get_user_by_id(id).await.unwrap();
//         Some(user)
//     }
//     async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
//         let users = ctx.data::<UserRepository>().unwrap().get_all_users().await.unwrap();
//         users
//     }
// }
