use serde::{Deserialize, Serialize};
use auth_sphere_db::table::user::UserKind;

/// 登录响应
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: u64,
    pub user_info: UserInfo,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginMfaResponse {
    pub requires_2fa: bool,
    pub token: String,
    pub expired: u64,
}

/// 用户信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub nickname: String,
    pub user_type: UserKind,
    pub is_mfa: bool,
    pub is_passkey: bool
}

/// OTP 发送响应
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OtpSendResponse {
    pub success: bool,
    pub message: String,
    pub expire_in: u64, // 验证码有效期（秒）
}

/// 注册响应
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub user_id: u64,
    pub username: String,
    pub message: String,
}
