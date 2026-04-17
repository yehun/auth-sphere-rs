use lib_mfa::{TotpConfig, TotpGenerator};
use std::path::PathBuf;
use std::fs;
use totp_rs::Algorithm;

const ISSUER: &'static str = "Auth-Sphere";
const ACCOUNT: &'static str = "yehun";

fn create_config() -> TotpConfig {
    TotpConfig {
        algorithm: Algorithm::SHA1,
        digits: 6,
        skew: 1,
        step: 30,
        issuer: Some(ISSUER.to_string()),
    }
    // TotpConfig::new(ISSUER.to_string())
}

#[test]
fn test_2fa() -> Result<(), Box<dyn std::error::Error>> {
    let secret = TotpGenerator::generate_secret();
    println!("Generated secret: {}", secret);

    // Create a custom configuration
    let config = create_config();

    // Create a TOTP generator
    let totp = TotpGenerator::new(config, &secret, ACCOUNT)
        .expect("Failed to create TOTP generator");

    // Generate current code
    let code = totp.generate_current().expect("Failed to generate code");
    println!("Current TOTP code: {}", code);

    // Verify a code
    let is_valid = totp.check(&code).expect("Verification failed");
    println!("Code is valid: {}", is_valid);

    // Get provisioning URI
    let uri = totp.get_uri();
    println!("URI: {}", uri);

    // Generate QR code as SVG
    let qr_png_base64 = totp.get_qr_png_base64().expect("Failed to generate QR code");
    println!("QR Code base64:\n{}", qr_png_base64);

    let qr_png = totp.get_qr_png()?;
    let qr_path: PathBuf = "/home/yehun/2fa.png".into();
    fs::write(qr_path, qr_png).expect("写入文件失败");

    // Get issuer and account info
    println!("Issuer: {}", totp.issuer().unwrap_or("Unknown"));
    println!("Account: {}", totp.account());
    Ok(())
}

/*
Generated secret: 4QZ4AEVQ7DPUZ7XK3CJLNES6JKLGVJQH
Current TOTP code: 879352
Code is valid: true
URI: otpauth://totp/Test:test?secret=4QZ4AEVQ7DPUZ7XK3CJLNES6JKLGVJQH&algorithm=SHA256&issuer=Test&period=120
QR Code base64:
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
Issuer: Test
Account: test
*/

/*
Generated secret: SECGZRNVWUNUUGXUFK5TZNM26FEIUKMK
Current TOTP code: 802062
Code is valid: true
URI: otpauth://totp/Auth-Sphere:yehun?secret=SECGZRNVWUNUUGXUFK5TZNM26FEIUKMK&issuer=Auth-Sphere
QR Code base64:
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
Issuer: Auth-Sphere
Account: yehun
*/

#[test]
fn test_verify() -> Result<(), Box<dyn std::error::Error>> {
    let secret = "SECGZRNVWUNUUGXUFK5TZNM26FEIUKMK";
    println!("Generated secret: {}", secret);
    let config = create_config();
    // let config = TotpConfig {
    //     algorithm: Algorithm::SHA256,
    //     digits: 6,
    //     skew: 1,
    //     step: 120,
    //     issuer: Some("Auth-Sphere".to_string()),
    // };
    let totp = TotpGenerator::new(config, &secret, ACCOUNT)
        .expect("Failed to create TOTP generator");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = "643135";
    let r = totp.check(input).expect("无效的code");
    println!("verify: {}", r);
    Ok(())

}