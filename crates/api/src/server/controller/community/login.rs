use actix_web::{web, Responder};
use actix_web_validator::Json;
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::controller::base::LoginController;
use crate::server::model::request::{CommunityLoginRequest, MfaLoginRequest, RegisterRequest};

/// Community 登录控制器实现
struct CommunityLoginController;

impl LoginController for CommunityLoginController {
    fn user_kind() -> UserKind {
        UserKind::Community
    }
}

/// 社区运营注册
pub async fn register(
    state: web::Data<AppState>,
    req: Json<RegisterRequest>,
) -> impl Responder {
    CommunityLoginController::register(state, req.into_inner()).await
}

/// 社区运营登录
pub async fn login_with_password(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<CommunityLoginRequest>,
) -> impl Responder {
    CommunityLoginController::login_with_password(
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
    CommunityLoginController::login_with_2fa(
        state,
        request,
        req.into_inner(),
    ).await
}

/// 社区运营登出
pub async fn logout(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    CommunityLoginController::logout(state, request).await
}

