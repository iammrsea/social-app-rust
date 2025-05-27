use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
    response::{IntoResponse, Response},
};
use shared::auth::{
    AuthUser,
    jwt::{Claims, verify_jwt},
};

#[derive(Debug, Clone)]
pub struct AxumAuthUser(pub Claims);

impl<S> FromRequestParts<S> for AxumAuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        if auth_header.is_none() {
            return Ok(AxumAuthUser(Claims::guest_claims()));
        }
        let token = auth_header.unwrap();
        let jwt_result = verify_jwt(token);
        if let Err(..) = jwt_result {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid or missing JWT".to_string(),
            )
                .into_response());
        }
        let claims = jwt_result.unwrap();
        Ok(AxumAuthUser(claims))
    }
}

impl From<AxumAuthUser> for AuthUser {
    fn from(value: AxumAuthUser) -> Self {
        AuthUser(value.0)
    }
}
