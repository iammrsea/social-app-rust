use crate::{
    auth_utils,
    errors::AuthError,
    otp::{
        OtpEntry,
        otp_respository::OtpRepository,
        utils::{compare_otps, generate_otp, get_otp_expiration, hash_otp},
    },
    result::AuthResult,
};

pub struct AuthService {
    otp_repo: Box<dyn OtpRepository>,
}

impl AuthService {
    pub fn new(otp_repo: Box<dyn OtpRepository>) -> Self {
        Self { otp_repo }
    }
    pub async fn generate_otp(&self, email: &str) -> AuthResult<String> {
        auth_utils::validate_email(email)?;
        let otp_val = generate_otp();
        let otp_hash = hash_otp(&otp_val);
        let expires_at = get_otp_expiration();
        let otp_entry =
            OtpEntry::new_with_all_fields(email.to_string(), false, 0, otp_hash, expires_at);
        self.otp_repo.upsert_otp(otp_entry).await?;
        Ok(otp_val)
    }

    pub async fn verify_otp(&self, email: &str, otp: &str) -> AuthResult<()> {
        auth_utils::validate_email(email)?;
        let mut otp_entry = self.otp_repo.get_otp_by_user_email(email).await?;

        if otp_entry.is_used() {
            return Err(AuthError::OtpAlreadyUsed);
        }
        if otp_entry.is_expired() {
            return Err(AuthError::OtpExpired);
        }
        if otp_entry.exceeded_attempts() {
            return Err(AuthError::TooManyAttempts);
        }
        if compare_otps(otp, otp_entry.otp_hash()) {
            otp_entry.mark_as_used();
            otp_entry.increment_attempts();
            self.otp_repo.upsert_otp(otp_entry).await?;
            Ok(())
        } else {
            otp_entry.increment_attempts();
            self.otp_repo.upsert_otp(otp_entry).await?;
            return Err(AuthError::InvalidOtp);
        }
    }
}
