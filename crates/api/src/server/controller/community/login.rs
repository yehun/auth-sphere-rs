use actix_http::HttpMessage;
use actix_web::{web, Responder};
use actix_web_validator::Json;
use tracing::{debug, error};
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::middleware::{Authorization, DeviceType};
use crate::server::model::request::{CommunityLoginRequest, RegisterRequest};
use crate::server::model::response::base::result::ResponseResult;


pub async fn register(
    state: web::Data<AppState>,
    req: Json<RegisterRequest>,
) -> impl Responder {
    debug!("Community register request: {:?}", req);
    match state.auth_service.register(
        &req.nickname,
        &req.username,
        &req.password,
        UserKind::Community,
        req.email.clone(),
        req.phone.clone(),
    ).await {
        Ok(response) => ResponseResult::success_with_data(response),
        Err(e) => {
            error!("Community registration failed: {}", e);
            ResponseResult::fail_with_message(&format!("注册失败: {}", e))
        }
    }
}

/// 社区运营登录
pub async fn login_with_password(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<CommunityLoginRequest>,
) -> impl Responder {
    debug!("Community login request: {:?}", req);
    let ext = request.extensions();
    let device = ext.get::<DeviceType>()
        .unwrap_or(&DeviceType::Unknown)
        .clone();
    let user_device = device.into();
    debug!("user device: {:?}", user_device);
    match state.auth_service.login_with_password(
        &req.username,
        &req.password,
        // req.device.clone().unwrap_or(UserDevice::Web),
        user_device,
        UserKind::Community
    ).await {
        Ok((response, true)) => ResponseResult::success_with_data(response),
        Ok((response, false)) => ResponseResult::success_with_data(response),
        Err(e) => {
            error!("Community login failed: {}", e);
            ResponseResult::fail_with_message(&format!("登录失败: {}", e))
        }
    }
}


pub async fn logout(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    let ext = request.extensions();
    let token = match ext.get::<Authorization>() {
        Some(t) => &t.0,
        None => return ResponseResult::<()>::fail_with_message("未提供认证令牌"),
    };

    match state.auth_service.logout(UserKind::Community, token).await {
        Ok(_) => ResponseResult::success_with_message("登出成功"),
        Err(e) => {
            error!("Logout failed: {}", e);
            ResponseResult::fail_with_message(&format!("登出失败: {}", e))
        }
    }
}

