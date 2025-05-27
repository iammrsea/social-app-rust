use crate::app_service::AppService;
use async_graphql::{Context, Object, SimpleObject};
use shared::{
    auth::{AppContext, AuthUser},
    command_handler::CommandHanlder,
    guards::roles::UserRole,
};

use user::{
    app::command::{
        sign_in::SignIn, sign_up::SignUp, verify_email_with_otp::VerifyEmailWithOtp,
        verify_otp::VerifyOtp,
    },
    domain::{errors::UserDomainError, result::UserDomainResult},
};

#[derive(SimpleObject, Debug, Default)]
pub struct AuthResponse {
    pub message: String,
}

#[derive(SimpleObject, Debug, Default)]
pub struct VerificationResponse {
    pub token: String,
}

#[derive(Debug, Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    #[graphql(name = "signUp")]
    async fn sign_up(&self, ctx: &Context<'_>, cmd: SignUp) -> UserDomainResult<AuthResponse> {
        let app_service = ctx.data::<AppService>().unwrap();
        //TODO: get user from context
        let app_ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Guest));
        app_service
            .services
            .user_service
            .command_handler
            .sign_up
            .handle(&app_ctx, cmd)
            .await?;

        Ok(AuthResponse {
            message: "Sign up was successful. Please check your email for the OTP.".to_string(),
        })
    }
    #[graphql(name = "signIn")]
    async fn sign_in(&self, ctx: &Context<'_>, cmd: SignIn) -> UserDomainResult<AuthResponse> {
        let app_service = ctx.data::<AppService>().unwrap();
        //TODO: get user from context
        let app_ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Guest));
        app_service
            .services
            .user_service
            .command_handler
            .sign_in
            .handle(&app_ctx, cmd)
            .await?;

        Ok(AuthResponse {
            message: "Please check your email for the OTP to complete your sign in process."
                .to_string(),
        })
    }

    #[graphql(name = "verifyOtp")]
    async fn verify_otp(
        &self,
        ctx: &Context<'_>,
        cmd: VerifyOtp,
    ) -> UserDomainResult<VerificationResponse> {
        let app_service = ctx.data::<AppService>().unwrap();
        //TODO: get user from context
        let app_ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Guest));
        let token = app_service
            .services
            .user_service
            .command_handler
            .verify_otp
            .handle(&app_ctx, cmd)
            .await?;

        Ok(VerificationResponse { token })
    }

    #[graphql(name = "verifyEmail")]
    async fn verify_email(
        &self,
        ctx: &Context<'_>,
        cmd: VerifyEmailWithOtp,
    ) -> UserDomainResult<VerificationResponse> {
        let app_service = ctx.data::<AppService>().unwrap();
        //TODO: get user from context
        let app_ctx = AppContext::new().with_user(AuthUser::new_test_auth_user(UserRole::Guest));
        let token = app_service
            .services
            .user_service
            .command_handler
            .verify_email_with_opt
            .handle(&app_ctx, cmd)
            .await?
            .ok_or(UserDomainError::UnableToVerifyEmail)?;

        Ok(VerificationResponse { token })
    }
}
