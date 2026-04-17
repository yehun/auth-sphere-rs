mod mfa;
pub use mfa::UserMfaService;

pub mod auth;
pub use auth::AuthService;

pub mod user;
pub use user::UserService;

mod passkey;
pub use passkey::UserPassKeyService;

use auth_sphere_db::table::user::UserKind;
use auth_sphere_db::table::user_session::UserDevice;

pub(crate) enum LoginType {
    Username,
    Email,
    Phone
}

pub(crate) fn login_type_from_str(s: &str) -> LoginType {
    if crate::utils::regex::is_email(&s) {
        return LoginType::Email;
    }
    if crate::utils::regex::is_phone(&s) {
        return LoginType::Phone;
    }
    LoginType::Username
}



pub(crate) fn generate_otp_key(user_type: &UserKind, user_device: &UserDevice, token: &str) -> String {
    let user_type = match user_type {
        UserKind::Member => "member",
        UserKind::Community => "community",
        UserKind::Platform => "platform",
    };
    let device = match user_device {
        UserDevice::Desktop => "desktop",
        UserDevice::Web => "web",
        UserDevice::Android => "android",
        UserDevice::Ios => "ios",
    };
    format!("login::otp::{}::{}::{}", user_type, device, token)
}

pub(crate) fn generate_login_key(user_type: &UserKind, token: &str) -> String {
    let user_type = match user_type {
        UserKind::Member => "member",
        UserKind::Community => "community",
        UserKind::Platform => "platform",
    };
    format!("login::{}::{}", user_type, token)
}


pub(crate) fn generate_mfa_key(user_type: &UserKind, token: &str) -> String {
    let user_type = match user_type {
        UserKind::Member => "member",
        UserKind::Community => "community",
        UserKind::Platform => "platform",
    };
    format!("login::mfa::{}::{}", user_type, token)
}

