use actix_web::{web, Responder};
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::controller::base::MfaController;

/// Member MFA 控制器实现
struct MemberMfaController;

impl MfaController for MemberMfaController {
    fn user_kind() -> UserKind {
        UserKind::Member
    }
}

/// 生成 MFA QR 码
pub async fn generate(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    MemberMfaController::generate_mfa(state, request).await
}

/// 激活 MFA
pub async fn active(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    MemberMfaController::activate_mfa(state, request).await
}

/// 停用 MFA
pub async fn deactive(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    MemberMfaController::deactivate_mfa(state, request).await
}