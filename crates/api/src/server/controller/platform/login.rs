use actix_http::HttpMessage;
use actix_web::{web, Responder};
use actix_web_validator::Json;
use tracing::{debug, error};
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::middleware::{Authorization, DeviceType};
use crate::server::model::request::{PlatformLoginRequest, RegisterRequest};
use crate::server::model::response::base::result::ResponseResult;

/// 平台运营登录
pub async fn login_with_password(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<PlatformLoginRequest>,
) -> impl Responder {
    debug!("Platform login request: {:?}", req);

    let ext = request.extensions();
    let device = ext.get::<DeviceType>()
        .unwrap_or(&DeviceType::Unknown)
        .clone();
    let user_device = device.into();
    debug!("user device: {:?}", user_device);
    match state.auth_service.login_with_password(
        &req.username,
        &req.password,
        user_device,
        UserKind::Platform
    ).await {
        Ok((response, true)) => ResponseResult::success_with_data(response),
        Ok((response, false)) => ResponseResult::success_with_data(response),
        Err(e) => {
            error!("Platform login failed: {}", e);
            ResponseResult::fail_with_message(&format!("登录失败: {}", e))
        }
    }
}

/// 平台运营注册
pub async fn register(
    state: web::Data<AppState>,
    req: Json<RegisterRequest>,
) -> impl Responder {
    debug!("Platform register request: {:?}", req);

    match state.auth_service.register(
        &req.nickname,
        &req.username,
        &req.password,
        UserKind::Platform,
        req.email.clone(),
        req.phone.clone(),
    ).await {
        Ok(response) => ResponseResult::success_with_data(response),
        Err(e) => {
            error!("Platform registration failed: {}", e);
            ResponseResult::fail_with_message(&format!("注册失败: {}", e))
        }
    }
}

/// 平台运营登出
pub async fn logout(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    let ext = request.extensions();
    let token = match ext.get::<Authorization>() {
        Some(t) => &t.0,
        None => return ResponseResult::<()>::fail_with_message("未提供认证令牌"),
    };

    match state.auth_service.logout(UserKind::Platform, token).await {
        Ok(_) => ResponseResult::success_with_message("登出成功"),
        Err(e) => {
            error!("Logout failed: {}", e);
            ResponseResult::fail_with_message(&format!("登出失败: {}", e))
        }
    }
}
