use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use shared::{config::Config, guards::roles::UserRole};

use serde::{Deserialize, Serialize};

use super::result::UserAuthResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: UserRole,
}

pub fn create_jwt(email: String, role: UserRole) -> UserAuthResult<String> {
    const JWT_EXPIRATION_IN_HOURS: i64 = 24;
    let expiration = Utc::now() + Duration::hours(JWT_EXPIRATION_IN_HOURS);
    let claims = Claims {
        sub: email,
        exp: expiration.timestamp() as usize,
        role,
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

pub fn verify_jwt(token: &str) -> UserAuthResult<Claims> {
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
    use shared::guards::roles::UserRole;

    #[test]
    fn test_create_jwt_and_verify() {
        let code = create_jwt("user@example.com".to_string(), UserRole::Admin).unwrap();

        let claims = verify_jwt(&code).unwrap();
        assert_eq!(claims.sub, "user@example.com");
        assert_eq!(claims.role, UserRole::Admin);
    }
}
