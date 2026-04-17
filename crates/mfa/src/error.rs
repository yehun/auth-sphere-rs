use thiserror::Error;

#[derive(Debug, Error)]
pub enum TotpError {
    #[error("Invalid uri")]
    InvalidURI,
    #[error("Invalid secret key")]
    InvalidSecret,
    #[error("Failed to generate TOTP code")]
    GenerationFailed,
    #[error("Verification failed")]
    VerificationFailed,
    #[error("QR code generation failed")]
    QrGenerationFailed,
    #[error("ttl system time error")]
    SystemTimeError,
}