use actix_http::HttpMessage;
use auth_sphere_db::table::user::{User, UserKind};
use auth_sphere_db::table::user_session::UserDevice;
use crate::server::middleware::DeviceType;
use crate::server::middleware::Authorization;

/// 从请求中提取认证令牌
pub fn extract_token(request: &actix_web::HttpRequest) -> Option<String> {
    request.extensions()
        .get::<Authorization>()
        .map(|auth| auth.0.clone())
}

/// 从请求中提取用户信息
pub async fn extract_user(
    state: &actix_web::web::Data<crate::config::AppState>,
    request: &actix_web::HttpRequest,
    user_kind: UserKind,
) -> Result<User, String> {
    let token = extract_token(request)
        .ok_or_else(|| "未提供认证令牌".to_string())?;

    // 获取当前用户信息
    let user_info = state.user_service
        .current_user(&user_kind, &token)
        .await
        .map_err(|e| format!("获取用户信息失败: {}", e))?;

    // 获取用户详情
    let user = state.user_service
        .get_user(user_kind, user_info.id)
        .await
        .map_err(|e| format!("获取用户详情失败: {}", e))?;

    Ok(user)
}

/// 从请求中提取设备类型
pub fn extract_device(request: &actix_web::HttpRequest) -> UserDevice {
    let device_type = request.extensions()
        .get::<DeviceType>()
        .unwrap_or(&DeviceType::Unknown)
        .clone();

    device_type.into()
}
