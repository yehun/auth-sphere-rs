use serde::{Deserialize, Serialize};
use validator::Validate;

/// 会员登录请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct MemberLoginRequest {
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
    #[validate(length(min = 6, message = "密码长度至少为6位"))]
    pub password: String,
}

/// 社区运营登录请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct CommunityLoginRequest {
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
    #[validate(length(min = 6, message = "密码长度至少为6位"))]
    pub password: String,
}

/// 平台运营登录请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct PlatformLoginRequest {
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
    #[validate(length(min = 6, message = "密码长度至少为6位"))]
    pub password: String,
}

/// OTP 登录请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct OtpLoginRequest {
    #[validate(length(min = 1, message = "手机号或邮箱不能为空"))]
    pub contact: String, // phone or email
    #[validate(length(min = 6, max = 6, message = "验证码必须为6位"))]
    pub otp_code: String,
}

/// 发送 OTP 请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct SendOtpRequest {
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    #[validate(regex(path = *crate::utils::regex::PHONE_REGEX, message = "手机号格式不正确"))]
    pub phone: Option<String>,
}

/// 注册请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct RegisterRequest {
    #[validate(length(min = 1, max = 5, message = "名称在1-5位之间"))]
    pub nickname: String,
    #[validate(length(min = 5, max = 20, message = "用户名不能为空"))]
    pub username: String,
    #[validate(length(min = 6, max = 32, message = "密码长度至少为6-32位"))]
    pub password: String,
    /// 用户类型: member, community, platform
    pub user_type: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// MFA 启用请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct MfaLoginRequest {
    pub token: String,
    #[validate(length(min = 6, max = 6))]
    pub code: String,
}

/// Passkey 注册开始请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct PasskeyRegisterBeginRequest {
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
}

/// Passkey 注册完成请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct PasskeyRegisterCompleteRequest {
    pub username: String,
    pub credential: serde_json::Value,
}

/// Passkey 登录开始请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct PasskeyLoginBeginRequest {
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
}

/// Passkey 登录完成请求
#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct PasskeyLoginCompleteRequest {
    pub username: String,
    pub credential: serde_json::Value,
}
