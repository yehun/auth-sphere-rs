use actix_web::{web, Responder};
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::controller::base::MfaController;

/// Community MFA 控制器实现
struct CommunityMfaController;

impl MfaController for CommunityMfaController {
    fn user_kind() -> UserKind {
        UserKind::Community
    }
}

/// 生成 MFA QR 码
pub async fn generate(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    CommunityMfaController::generate_mfa(state, request).await
}

/// 激活 MFA
pub async fn active(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    CommunityMfaController::activate_mfa(state, request).await
}

/// 停用 MFA
pub async fn deactive(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    CommunityMfaController::deactivate_mfa(state, request).await
}
