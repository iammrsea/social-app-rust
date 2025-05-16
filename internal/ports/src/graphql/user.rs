use crate::app_service::AppService;
use async_graphql::{Context, Object};
use shared::{
    auth::{AppContext, AuthUser},
    command_handler::CommandHanlder,
    guards::roles::UserRole,
    query_handler::QueryHandler,
    types::AppResult,
};
use user::app::{
    command::create_account::CreateAccount,
    query::{user_by_email::GetUserByEmail, user_by_id::GetUserById},
};
use user::domain::user_read_model::UserReadModel;

#[derive(Default, Debug)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(name = "getUserById")]
    async fn get_user_by_id(&self, ctx: &Context<'_>, id: String) -> AppResult<UserReadModel> {
        let app_service = ctx.data::<AppService>().unwrap();
        let app_ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));
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
    ) -> AppResult<UserReadModel> {
        let app_service = ctx.data::<AppService>().unwrap();
        let app_ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Admin));
        app_service
            .services
            .user_service
            .query_handler
            .get_user_by_email
            .handle(&app_ctx, GetUserByEmail { email })
            .await
    }
}

#[derive(Debug, Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(name = "createAccount")]
    async fn create_account(
        &self,
        ctx: &Context<'_>,
        cmd: CreateAccount,
    ) -> AppResult<UserReadModel> {
        let app_service = ctx.data::<AppService>().unwrap();
        //TODO: get user from context
        let app_ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Guest));
        app_service
            .services
            .user_service
            .command_handler
            .create_account
            .handle(&app_ctx, cmd.clone())
            .await?;

        let user = app_service
            .services
            .user_service
            .query_handler
            .get_user_by_email
            .handle(&app_ctx, GetUserByEmail { email: cmd.email })
            .await?;
        Ok(user)
    }
}
