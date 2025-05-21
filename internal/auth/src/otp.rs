use chrono::{Duration, Utc};
use getset::Getters;

pub mod mongoimpl;
pub mod otp_respository;

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
    pub fn new(email: String) -> Self {
        let otp = utils::generate_otp();
        let otp_hash = utils::hash_otp(&otp);
        let expires_at = (Utc::now() + Duration::minutes(5)).timestamp_millis();
        Self {
            otp_hash,
            expires_at,
            used: false,
            attempts: 0,
            email,
        }
    }
    pub fn new_with_all_fields(
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
    pub fn compare_otps(otp: &str, otp_hash: &str) -> bool {
        let hashed_otp = hash_otp(otp);
        hashed_otp == otp_hash
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
            assert!(compare_otps(otp, &hashed_otp));
        }
    }
}
