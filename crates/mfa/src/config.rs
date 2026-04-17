use totp_rs::Algorithm;

#[derive(Debug, Clone)]
pub struct TotpConfig {
    pub algorithm: Algorithm,
    pub digits: usize,
    pub skew: u8,
    pub step: u64,
    pub issuer: Option<String>,
}

impl TotpConfig {
    pub fn new(issuer: String) -> Self {
        TotpConfig {
            issuer: Some(issuer),
            ..TotpConfig::default()
        }
    }
}

impl Default for TotpConfig {
    fn default() -> Self {
        TotpConfig {
            algorithm: Algorithm::SHA1,
            digits: 6,
            skew: 1,
            step: 30,
            issuer: None,
        }
    }
}