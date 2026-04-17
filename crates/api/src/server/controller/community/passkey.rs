use actix_http::HttpMessage;
use actix_web::{web, Responder, HttpResponse};
use actix_web_validator::Json;
use tracing::{debug, error};
use auth_sphere_db::table::user::{UserKind};
use crate::config::AppState;
use crate::server::middleware::Authorization;
use crate::server::model::request::{PasskeyRegisterBeginRequest, PasskeyRegisterCompleteRequest, PasskeyLoginBeginRequest, PasskeyLoginCompleteRequest};
use crate::server::model::response::base::result::ResponseResult;

/// 开始 Passkey 注册
pub async fn register_begin(
    state: web::Data<AppState>,
    req: Json<PasskeyRegisterBeginRequest>,
) -> impl Responder {
    debug!("Passkey register begin request: {:?}", req);
    
    // 根据用户名查找用户
    let user = match state.user_service.get_by_username(UserKind::Community, &req.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return ResponseResult::<()>::fail_with_message("用户不存在").response(),
        Err(e) => {
            error!("Failed to get user: {:?}", e);
            return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
        }
    };

    // 开始注册流程
    match state.passkey_service.register_begin(
        &req.username,
        user.id,
        &user.nickname,
    ).await {
        Ok(challenge) => {
            debug!("Passkey registration challenge: {:?}", challenge);
            HttpResponse::Ok()
                .content_type("application/json")
                .json(challenge)
        },
        Err(e) => {
            error!("Passkey register begin failed: {}", e);
            ResponseResult::<()>::fail_with_message(&e).response()
        }
    }
}

/// 完成 Passkey 注册
pub async fn register_complete(
    state: web::Data<AppState>,
    req: Json<PasskeyRegisterCompleteRequest>,
) -> impl Responder {
    debug!("Passkey register complete request for user: {}", req.username);
    
    // 根据用户名查找用户
    let user = match state.user_service.get_by_username(UserKind::Community, &req.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return ResponseResult::<()>::fail_with_message("用户不存在").response(),
        Err(e) => {
            error!("Failed to get user: {:?}", e);
            return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
        }
    };

    // 完成注册流程
    match state.passkey_service.register_complete(
        &req.username,
        user.id,
        req.credential.clone(),
    ).await {
        Ok(()) => ResponseResult::<()>::success().response(),
        Err(e) => {
            error!("Passkey register complete failed: {}", e);
            ResponseResult::<()>::fail_with_message(&e).response()
        }
    }
}

/// 开始 Passkey 登录
pub async fn login_begin(
    state: web::Data<AppState>,
    req: Json<PasskeyLoginBeginRequest>,
) -> impl Responder {
    debug!("Passkey login begin request: {:?}", req);
    
    // 根据用户名查找用户
    let user = match state.user_service.get_by_username(UserKind::Community, &req.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return ResponseResult::<()>::fail_with_message("用户不存在").response(),
        Err(e) => {
            error!("Failed to get user: {:?}", e);
            return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
        }
    };

    // 开始登录流程
    match state.passkey_service.login_begin(
        &req.username,
        user.id,
    ).await {
        Ok(challenge) => {
            debug!("Passkey login challenge: {:?}", challenge);
            HttpResponse::Ok().json(challenge)
        },
        Err(e) => {
            error!("Passkey login begin failed: {}", e);
            ResponseResult::<()>::fail_with_message(&e).response()
        }
    }
}

/// 完成 Passkey 登录
pub async fn login_complete(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    req: Json<PasskeyLoginCompleteRequest>,
) -> impl Responder {
    debug!("Passkey login complete request for user: {}", req.username);
    
    // 根据用户名查找用户
    let user = match state.user_service.get_by_username(UserKind::Community, &req.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return ResponseResult::<()>::fail_with_message("用户不存在").response(),
        Err(e) => {
            error!("Failed to get user: {:?}", e);
            return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
        }
    };

    // 完成登录流程
    match state.passkey_service.login_complete(
        &req.username,
        user.id,
        req.credential.clone(),
    ).await {
        Ok(()) => {
            use actix_http::HttpMessage;
            use crate::server::middleware::DeviceType;
            
            let ext = request.extensions();
            let device = ext.get::<DeviceType>()
                .unwrap_or(&DeviceType::Unknown)
                .clone();
            let user_device = device.into();
            
            match state.auth_service.create_session(&user, user_device).await {
                Ok(session) => {
                    let login_response = state.auth_service.session_to_login_response(session, &user);
                    ResponseResult::success_with_data(login_response).response()
                },
                Err(e) => {
                    error!("Failed to create session: {:?}", e);
                    ResponseResult::<()>::fail_with_message("生成令牌失败").response()
                }
            }
        },
        Err(e) => {
            error!("Passkey login complete failed: {}", e);
            ResponseResult::<()>::fail_with_message(&e).response()
        }
    }
}

/// 关闭 Passkey
pub async fn deactive(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> impl Responder {
    let ext = request.extensions();
    let token = match ext.get::<Authorization>() {
        Some(t) => &t.0,
        None => {
            return ResponseResult::<()>::fail_with_message("请先登陆").response();
        },
    };
    let Ok(user) = state.user_service.current_user(UserKind::Community, token).await else {
        return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
    };
    if let Err(e) = state.passkey_service.deactived(user.id).await {
        return ResponseResult::<()>::fail_with_message(&format!("关闭PassKey失败: {e}")).response();
    }
    ResponseResult::<()>::success().response()
}
