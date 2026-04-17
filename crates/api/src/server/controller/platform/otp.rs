use crate::config::AppState;
use crate::server::controller::base::OtpController;
use crate::server::model::request::{OtpLoginRequest, SendOtpRequest};
use actix_web::{web, Responder};
use actix_web_validator::Json;
use auth_sphere_db::table::user::UserKind;


struct PlatformOtpController;

impl OtpController for PlatformOtpController {
    fn user_kind() -> UserKind {
        UserKind::Platform
    }
}


/// 平台运营 OTP 登录
pub async fn login(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<OtpLoginRequest>,
) -> impl Responder {
    PlatformOtpController::otp_login(
        state,
        request,
        req.contact.clone(),
        req.otp_code.clone(),
    ).await
}

/// 发送 OTP
pub async fn send(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<SendOtpRequest>,
) -> impl Responder {
    PlatformOtpController::send_otp(
        state,
        request,
        req.email.clone(),
        req.phone.clone(),
    ).await
}
