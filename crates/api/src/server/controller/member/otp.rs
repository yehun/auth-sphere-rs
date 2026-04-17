use actix_http::HttpMessage;
use actix_web::{web, Responder};
use actix_web_validator::Json;
use tracing::{debug, error};
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::middleware::DeviceType;
use crate::server::model::request::{OtpLoginRequest, SendOtpRequest};
use crate::server::model::response::base::result::ResponseResult;

/// 会员 OTP 登录
pub async fn login(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<OtpLoginRequest>,
) -> impl Responder {
    debug!("Member OTP login request: {:?}", req);
    let ext = request.extensions();
    let device = ext.get::<DeviceType>()
        .unwrap_or(&DeviceType::Unknown)
        .clone();
    let user_device = device.into();
    debug!("user device: {:?}", user_device);

    match state.auth_service.login_with_code(
        &req.contact,
        &req.otp_code,
        user_device,
        UserKind::Member,
    ).await {
        Ok(response) => ResponseResult::success_with_data(response),
        Err(e) => {
            error!("OTP login failed: {}", e);
            ResponseResult::fail_with_message(&format!("OTP登录失败: {}", e))
        }
    }
}

/// 发送 OTP
pub async fn send(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<SendOtpRequest>,
) -> impl Responder {
    debug!("Send OTP request: {:?}", req);
    let ext = request.extensions();
    let device = ext.get::<DeviceType>()
        .unwrap_or(&DeviceType::Unknown)
        .clone();
    let user_device = device.into();
    debug!("user device: {:?}", user_device);

    match state.auth_service.send_otp(
        req.email.clone(),
        req.phone.clone(),
        user_device,
        UserKind::Member,
    ).await {
        Ok(response) => ResponseResult::success_with_data(response),
        Err(e) => {
            error!("Send OTP failed: {}", e);
            ResponseResult::fail_with_message(&format!("发送验证码失败: {}", e))
        }
    }
}