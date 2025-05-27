use jwt::Claims;

use crate::guards::roles::UserRole;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub user: Option<AuthUser>,
}

impl AppContext {
    pub fn new() -> Self {
        Self { user: None }
    }
    pub fn with_user(mut self, user: AuthUser) -> Self {
        self.user = Some(user);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthUser(pub Claims);

impl AuthUser {
    pub fn new(claims: Claims) -> Self {
        Self(claims)
    }
    pub fn guest() -> Self {
        Self(Claims::guest_claims())
    }
    pub fn new_test_auth_user(role: UserRole) -> Self {
        Self(Claims {
            exp: 0,
            sub: "johndoe@example.com".to_string(),
            role,
            id: "test-user-id".to_string(),
        })
    }
}

pub fn get_auth_user_from_ctx(ctx: &AppContext) -> &AuthUser {
    ctx.user.as_ref().unwrap()
}

pub mod jwt {
    use crate::{config::Config, guards::roles::UserRole};
    use chrono::{Duration, Utc};
    use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

    use serde::{Deserialize, Serialize};

    pub type JWTError = jsonwebtoken::errors::Error;
    pub type JWTResult<T> = Result<T, JWTError>;

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct Claims {
        /// Subject (usually the user's email)
        pub sub: String,
        pub exp: usize,
        pub role: UserRole,
        pub id: String,
    }

    impl Claims {
        pub fn guest_claims() -> Self {
            Self {
                sub: "".to_string(),
                exp: 0,
                role: UserRole::Guest,
                id: "".to_string(),
            }
        }
    }

    pub fn create_jwt(email: String, role: UserRole, id: String) -> JWTResult<String> {
        const JWT_EXPIRATION_IN_HOURS: i64 = 24;
        let expiration = Utc::now() + Duration::hours(JWT_EXPIRATION_IN_HOURS);
        let claims = Claims {
            sub: email,
            exp: expiration.timestamp() as usize,
            role,
            id,
        };

        let config = Config::build();
        let secret = config.auth_secret.as_bytes();

        let code = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret),
        )?;
        Ok(code)
    }

    pub fn verify_jwt(token: &str) -> JWTResult<Claims> {
        let config = Config::build();
        let secret = config.auth_secret.as_bytes();
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret),
            &Validation::new(Algorithm::HS256),
        )
        .map(|d| d.claims)?;
        Ok(claims)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_create_jwt_and_verify() {
            let code = create_jwt(
                "user@example.com".to_string(),
                UserRole::Admin,
                "user-id".to_string(),
            )
            .unwrap();

            let claims = verify_jwt(&code).unwrap();
            assert_eq!(claims.sub, "user@example.com");
            assert_eq!(claims.role, UserRole::Admin);
        }
    }
}
