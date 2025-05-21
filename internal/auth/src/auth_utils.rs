use serde::Deserialize;
use validator::Validate;

use crate::AuthResult;

pub fn validate_email(email: &str) -> AuthResult<()> {
    #[derive(Validate, Deserialize)]
    struct EmailValidator {
        #[validate(email)]
        email: String,
    }
    let email_validator = EmailValidator {
        email: email.to_string(),
    };
    email_validator.validate()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_is_valid() {
        let res = validate_email("user@gmail.com");
        assert!(res.is_ok());
    }
    #[test]
    fn test_email_is_invalid() {
        let res = validate_email("usergmail");
        assert!(res.is_err());
    }
}
