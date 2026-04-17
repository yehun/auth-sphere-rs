use actix_http::HttpMessage;
use actix_web::{web, Responder};
use actix_web_validator::Json;
use tracing::{debug, error};
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::middleware::{Authorization, DeviceType};
use crate::server::model::request::{CommunityLoginRequest, MfaLoginRequest, RegisterRequest};
use crate::server::model::response::auth::UserInfo;
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
        user_device,
        UserKind::Community
    ).await {
        Ok((response, false)) => ResponseResult::success_with_data(response).response(),
        Ok((response, true)) => {
            match state.mfa_service.login(response).await {
                Ok(res) => {
                    ResponseResult::success_with_data(res).response()
                }
                Err(e) => {
                    ResponseResult::<()>::fail_with_message(&format!("登录失败: {}", e)).response()
                }
            }
        },
        Err(e) => {
            ResponseResult::<()>::fail_with_message(&format!("登录失败: {}", e)).response()
        }
    }
}

/// 2FA 验证
pub async fn login_with_2fa(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<MfaLoginRequest>,
) -> impl Responder {
    debug!("Community 2FA login request: {:?}", req);

    let ext = request.extensions();
    let device = ext.get::<DeviceType>()
        .unwrap_or(&DeviceType::Unknown)
        .clone();
    let user_device = device.into();
    debug!("user device: {:?}", user_device);

    let temp_token = &req.token;
    let otp_code = &req.code;
    
    // 从 Redis 获取临时令牌对应的用户信息
    let mfa_key = crate::server::service::generate_mfa_key(&UserKind::Community, temp_token);
    let user_data_str = match state.redis.get(&mfa_key).await {
        Ok(data) => data,
        Err(_) => return ResponseResult::<()>::fail_with_message("临时令牌已过期，请重新登录").response(),
    };
    
    let user_data: UserInfo = match serde_json::from_str(&user_data_str) {
        Ok(v) => v,
        Err(_) => return ResponseResult::<()>::fail_with_message("临时令牌无效").response(),
    };

    let user_id = user_data.id;
    let username = user_data.username;

    // 验证 OTP 代码
    match state.mfa_service.check(user_id, &username, otp_code).await {
        Ok(true) => {
            // OTP 验证成功，删除临时令牌
            let _ = state.redis.delete(&mfa_key).await;
            let user = state.user_service.get_user(UserKind::Community, user_id).await.unwrap();
            match state.auth_service.create_session(&user, user_device).await {
                Ok(session) => {
                    let response = state.auth_service.session_to_login_response(session, &user);
                    ResponseResult::success_with_data(response).response()
                }
                Err(e) => {
                    error!("Failed to create session: {}", e);
                    ResponseResult::<()>::fail_with_message(&format!("创建会话失败: {}", e)).response()
                }
            }
        }
        Ok(false) => ResponseResult::<()>::fail_with_message("验证码错误").response(),
        Err(e) => {
            error!("MFA verification failed: {}", e);
            ResponseResult::<()>::fail_with_message(&format!("验证失败: {}", e)).response()
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

