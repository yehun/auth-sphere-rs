use actix_http::HttpMessage;
use actix_web::{web, HttpResponse};
use tracing::error;
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::middleware::Authorization;
use crate::server::model::response::base::result::ResponseResult;

/// 获取当前用户信息
pub async fn info(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> HttpResponse {
    let ext = request.extensions();
    let token = match ext.get::<Authorization>() {
        Some(t) => &t.0,
        None => {
            return ResponseResult::<()>::fail_with_message("请先登陆").response();
        },
    };
    match state.user_service.current_user(UserKind::Member, token).await {
        Ok(user_info) => ResponseResult::success_with_data(user_info).response(),
        Err(e) => {
            error!("Get current user failed: {}", e);
            ResponseResult::<()>::fail_with_message(&format!("获取用户信息失败: {}", e)).response()
        }
    }
}