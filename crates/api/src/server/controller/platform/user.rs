use actix_web::{web, HttpResponse};
use tracing::error;
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::model::response::base::result::ResponseResult;

/// 获取当前用户信息
pub async fn me(
    state: web::Data<AppState>,
    headers: actix_web::HttpRequest,
) -> HttpResponse {
    let token = headers
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t,
        None => return ResponseResult::<()>::fail_with_message("未提供认证令牌").response(),
    };

    match state.user_service.current_user(&UserKind::Platform, token).await {
        Ok(session) => ResponseResult::success_with_data(session).response(),
        Err(e) => {
            error!("Verify token failed: {}", e);
            ResponseResult::<()>::fail_with_message(&format!("令牌验证失败: {}", e)).response()
        }
    }
}