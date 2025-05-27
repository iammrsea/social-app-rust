use chrono::{Duration, Utc};
use getset::Getters;

use super::{errors::UserAuthError, result::UserAuthResult};

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct OtpEntry {
    otp_hash: String,
    expires_at: i64,
    used: bool,
    attempts: u32,
    email: String,
}

impl OtpEntry {
    pub fn new(
        email: String,
        used: bool,
        attempts: u32,
        otp_hash: String,
        expires_at: i64,
    ) -> Self {
        Self {
            otp_hash,
            expires_at,
            used,
            attempts,
            email,
        }
    }
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp_millis() > self.expires_at
    }
    pub fn is_used(&self) -> bool {
        self.used
    }
    pub fn exceeded_attempts(&self) -> bool {
        const MAX_ALLOWED_ATTEMPTS: u32 = 5;
        self.attempts >= MAX_ALLOWED_ATTEMPTS
    }
    pub fn mark_as_used(&mut self) {
        self.used = true;
    }
    pub fn increment_attempts(&mut self) {
        self.attempts += 1;
    }
    pub fn validate_otp(&self) -> UserAuthResult<()> {
        if self.is_used() {
            return Err(UserAuthError::OtpAlreadyUsed);
        }
        if self.is_expired() {
            return Err(UserAuthError::OtpExpired);
        }
        if self.exceeded_attempts() {
            return Err(UserAuthError::TooManyAttempts);
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ComparedOtps {
    Equal,
    NotEqual,
}

pub mod utils {
    use super::*;
    use rand::{Rng, distr::Uniform};
    use sha2::{Digest, Sha256};

    pub fn generate_otp() -> String {
        let mut rng = rand::rng();
        let range = Uniform::try_from(0..10).unwrap();
        (0..6).map(|_| rng.sample(&range).to_string()).collect()
    }
    pub fn hash_otp(otp: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(otp.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    pub fn compare_otps(otp: &str, otp_hash: &str) -> ComparedOtps {
        let hashed_otp = hash_otp(otp);
        if hashed_otp == otp_hash {
            ComparedOtps::Equal
        } else {
            ComparedOtps::NotEqual
        }
    }
    pub fn get_otp_expiration() -> i64 {
        let expiration_time = Utc::now() + Duration::minutes(5);
        expiration_time.timestamp_millis()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_generate_otp() {
            let otp = generate_otp();
            assert_eq!(otp.len(), 6);
        }
        #[test]
        fn test_hash_otp() {
            let otp = "123456";
            let hashed_otp = hash_otp(otp);
            assert_eq!(hashed_otp.len(), 64);
        }
        #[test]
        fn test_compare_otps() {
            let otp = "123456";
            let hashed_otp = hash_otp(otp);
            assert_eq!(compare_otps(otp, &hashed_otp), ComparedOtps::Equal);
        }
    }
}
