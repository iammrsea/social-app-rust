use crate::app_service::AppService;
use async_graphql::{Context, Object};
use shared::{
    auth::{AppContext, AuthUser},
    query_handler::QueryHandler,
};
use user::domain::user_read_model::UserReadModel;
use user::{
    app::query::{user_by_email::GetUserByEmail, user_by_id::GetUserById},
    domain::result::UserDomainResult,
};

#[derive(Default, Debug)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(name = "getUserById")]
    async fn get_user_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> UserDomainResult<UserReadModel> {
        let app_service = ctx.data::<AppService>().unwrap();
        let auth_user = ctx.data::<AuthUser>().unwrap();
        let app_ctx = AppContext::new().with_user(auth_user.to_owned());
        app_service
            .services
            .user_service
            .query_handler
            .get_user_by_id
            .handle(&app_ctx, GetUserById { id })
            .await
    }

    #[graphql(name = "getUserByEmail")]
    async fn get_user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> UserDomainResult<UserReadModel> {
        let app_service = ctx.data::<AppService>().unwrap();
        let auth_user = ctx.data::<AuthUser>().unwrap();
        let app_ctx = AppContext::new().with_user(auth_user.to_owned());
        app_service
            .services
            .user_service
            .query_handler
            .get_user_by_email
            .handle(&app_ctx, GetUserByEmail { email })
            .await
    }
}
