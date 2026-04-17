use lib_mfa::totp_rs::Algorithm;
use lib_mfa::TotpConfig;
use crate::config::application;

pub fn init() -> TotpConfig {
    let config = application::get();
    TotpConfig {
        algorithm: Algorithm::SHA1,
        digits: 6,
        skew: 1,
        step: 30,
        issuer: Some(config.app.name),
    }
}