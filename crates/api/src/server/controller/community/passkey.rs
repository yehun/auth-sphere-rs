use actix_web::{web, Responder};
use actix_web_validator::Json;
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::controller::base::PasskeyController;
use crate::server::model::request::{PasskeyRegisterBeginRequest, PasskeyRegisterCompleteRequest, PasskeyLoginBeginRequest, PasskeyLoginCompleteRequest};

/// Community Passkey 控制器实现
struct CommunityPasskeyController;

impl PasskeyController for CommunityPasskeyController {
    fn user_kind() -> UserKind {
        UserKind::Community
    }
}

/// 开始 Passkey 注册
pub async fn register_begin(
    state: web::Data<AppState>,
    req: Json<PasskeyRegisterBeginRequest>,
) -> impl Responder {
    CommunityPasskeyController::passkey_register_begin(
        state,
        req.username.clone(),
    ).await
}

/// 完成 Passkey 注册
pub async fn register_complete(
    state: web::Data<AppState>,
    req: Json<PasskeyRegisterCompleteRequest>,
) -> impl Responder {
    CommunityPasskeyController::passkey_register_complete(
        state,
        req.username.clone(),
        req.credential.clone(),
    ).await
}

/// 开始 Passkey 登录
pub async fn login_begin(
    state: web::Data<AppState>,
    req: Json<PasskeyLoginBeginRequest>,
) -> impl Responder {
    CommunityPasskeyController::passkey_login_begin(
        state,
        req.username.clone(),
    ).await
}

/// 完成 Passkey 登录
pub async fn login_complete(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<PasskeyLoginCompleteRequest>,
) -> impl Responder {
    CommunityPasskeyController::passkey_login_complete(
        state,
        request,
        req.username.clone(),
        req.credential.clone(),
    ).await
}

/// 停用 Passkey
pub async fn deactive(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    CommunityPasskeyController::passkey_deactivate(state, request).await
}
