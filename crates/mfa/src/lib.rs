pub mod totp_rs {
    pub use totp_rs::*;
}

mod error;
mod config;

pub use crate::config::TotpConfig;
use crate::error::TotpError;
use base32;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Secret, TOTP};

#[derive(Debug)]
pub struct TotpGenerator {
    totp: TOTP,
}

impl TryFrom<String> for TotpGenerator {
    type Error = TotpError;

    fn try_from(url: String) -> Result<Self, Self::Error> {
        let totp = TOTP::from_url(url).map_err(|_| TotpError::InvalidURI)?;
        Ok(TotpGenerator { totp })
    }
}

impl TotpGenerator {

    pub fn new(
        config: TotpConfig,
        secret: &str,
        account: &str,
    ) -> Result<Self, TotpError> {
        let secret_bytes = base32::decode(
            base32::Alphabet::RFC4648 { padding: true },
            secret
        ).ok_or(TotpError::InvalidSecret)?;

        let secret = Secret::Raw(secret_bytes).to_bytes()
            .map_err(|_| TotpError::InvalidSecret)?;

        let totp = Self::create_totp(config, secret, account)?;

        Ok(TotpGenerator { totp })
    }

    fn create_totp(config: TotpConfig, secret: Vec<u8>, account: &str) -> Result<TOTP,  TotpError> {
        TOTP::new(
            config.algorithm,
            config.digits,
            config.skew,
            config.step,
            secret,
            config.issuer,
            account.to_string(),
        ).map_err(|_| TotpError::InvalidSecret)
    }

    pub fn generate_secret() -> String {
        let mut rng = rand::thread_rng();
        let mut secret = vec![0u8; 20];
        rng.fill(&mut secret[..]);
        base32::encode(
            base32::Alphabet::RFC4648 { padding: true },
            &secret
        )
    }

    pub fn generate_current(&self) -> Result<String, TotpError> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| TotpError::GenerationFailed)?
            .as_secs();

        Ok(self.totp.generate(time))
    }

    pub fn check(&self, code: &str) -> Result<bool, TotpError> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| TotpError::VerificationFailed)?
            .as_secs();
        Ok(self.totp.check(code, time))
    }

    pub fn check_current(&self, code: &str) -> Result<bool, TotpError> {
        self.totp.check_current(code).map_err(|_| TotpError::VerificationFailed)
    }

    pub fn get_ttl(&self) -> Result<u64, TotpError> {
        self.totp.ttl().map_err(|_| TotpError::SystemTimeError)
    }

    pub fn get_uri(&self) -> String {
        self.totp.get_url()
    }

    pub fn get_qr_png(&self) -> Result<Vec<u8>, TotpError> {
        self.totp.get_qr_png().map_err(|_| TotpError::QrGenerationFailed)
    }

    pub fn get_qr_png_base64(&self) -> Result<String, TotpError> {
        self.totp.get_qr_base64().map_err(|_| TotpError::QrGenerationFailed)
    }

    pub fn issuer(&self) -> Option<&str> {
        self.totp.issuer.as_deref()
    }

    pub fn account(&self) -> &str {
        &self.totp.account_name
    }
}