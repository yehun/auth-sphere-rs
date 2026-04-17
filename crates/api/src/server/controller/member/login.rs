use actix_web::{web, Responder};
use actix_web_validator::Json;
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::controller::base::LoginController;
use crate::server::model::request::{MemberLoginRequest, MfaLoginRequest, RegisterRequest};

/// Member 登录控制器实现
struct MemberLoginController;

impl LoginController for MemberLoginController {
    fn user_kind() -> UserKind {
        UserKind::Member
    }
}

/// 会员登录
pub async fn login_with_password(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<MemberLoginRequest>,
) -> impl Responder {
    MemberLoginController::login_with_password(
        state,
        request,
        req.username.clone(),
        req.password.clone(),
    ).await
}

/// 2FA 验证
pub async fn login_with_2fa(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<MfaLoginRequest>,
) -> impl Responder {
    MemberLoginController::login_with_2fa(
        state,
        request,
        req.into_inner(),
    ).await
}

/// 会员注册
pub async fn register(
    state: web::Data<AppState>,
    req: Json<RegisterRequest>,
) -> impl Responder {
    MemberLoginController::register(
        state,
        req.into_inner(),
    ).await
}

/// 会员登出
pub async fn logout(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    MemberLoginController::logout(state, request).await
}
