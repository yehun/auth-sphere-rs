use actix_web::{web, Responder};
use actix_web_validator::Json;
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::controller::base::OtpController;
use crate::server::model::request::{OtpLoginRequest, SendOtpRequest};

/// Member OTP 控制器实现
struct MemberOtpController;

impl OtpController for MemberOtpController {
    fn user_kind() -> UserKind {
        UserKind::Member
    }
}

/// 会员 OTP 登录
pub async fn login(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<OtpLoginRequest>,
) -> impl Responder {
    MemberOtpController::otp_login(
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
    MemberOtpController::send_otp(
        state,
        request,
        req.email.clone(),
        req.phone.clone(),
    ).await
}