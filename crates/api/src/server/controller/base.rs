use actix_web::{web, HttpRequest};
use tracing::debug;
use webauthn_rs::prelude::{PublicKeyCredential, RegisterPublicKeyCredential};
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::controller::helper::{extract_device, extract_token};
use crate::server::model::request::{MfaLoginRequest, RegisterRequest};
use crate::server::model::response::base::result::ResponseResult;

/// 通用登录控制器 trait
pub trait LoginController {
    /// 获取用户类型
    fn user_kind() -> UserKind;
    
    /// 密码登录
    async fn login_with_password(
        state: web::Data<AppState>,
        request: HttpRequest,
        username: String,
        password: String,
    ) -> actix_web::HttpResponse {
        debug!("{:?} login request", Self::user_kind());
        
        let user_device = extract_device(&request);
        debug!("user device: {:?}", user_device);
        
        match state.auth_service.login_with_password(
            &username,
            &password,
            user_device,
            Self::user_kind(),
        ).await {
            Ok((response, false)) => {
                // 不需要 2FA，直接返回
                ResponseResult::success_with_data(response).response()
            }
            Ok((response, true)) => {
                // 需要 2FA
                match state.mfa_service.login(response).await {
                    Ok(res) => ResponseResult::success_with_data(res).response(),
                    Err(e) => {
                        tracing::error!("MFA login failed: {}", e);
                        ResponseResult::<()>::fail_with_message(&format!("登录失败: {}", e)).response()
                    }
                }
            }
            Err(e) => {
                tracing::error!("Login failed: {}", e);
                ResponseResult::<()>::fail_with_message(&format!("登录失败: {}", e)).response()
            }
        }
    }
    
    /// 2FA 验证
    async fn login_with_2fa(
        state: web::Data<AppState>,
        request: HttpRequest,
        req: MfaLoginRequest,
    ) -> actix_web::HttpResponse {
        debug!("{:?} 2FA login request", Self::user_kind());
        
        let user_device = extract_device(&request);
        let temp_token = &req.token;
        let otp_code = &req.code;
        
        // 从 Redis 获取临时令牌对应的用户信息
        let mfa_key = crate::server::service::generate_mfa_key(&Self::user_kind(), temp_token);
        let user_data_str = match state.redis.get(&mfa_key).await {
            Ok(data) => data,
            Err(_) => return ResponseResult::<()>::fail_with_message("临时令牌已过期，请重新登录").response(),
        };
        
        use crate::server::model::response::auth::UserInfo;
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
                
                let user = match state.user_service.get_user(Self::user_kind(), user_id).await {
                    Ok(u) => u,
                    _ => return ResponseResult::<()>::fail_with_message("用户不存在").response(),
                };
                
                match state.auth_service.create_session(&user, user_device).await {
                    Ok(session) => {
                        let response = state.auth_service.session_to_login_response(session, &user);
                        ResponseResult::success_with_data(response).response()
                    }
                    Err(e) => {
                        tracing::error!("Failed to create session: {}", e);
                        ResponseResult::<()>::fail_with_message(&format!("创建会话失败: {}", e)).response()
                    }
                }
            }
            Ok(false) => ResponseResult::<()>::fail_with_message("验证码错误").response(),
            Err(e) => {
                tracing::error!("MFA verification failed: {}", e);
                ResponseResult::<()>::fail_with_message(&format!("验证失败: {}", e)).response()
            }
        }
    }
    
    /// 注册
    async fn register(
        state: web::Data<AppState>,
        req: RegisterRequest,
    ) -> actix_web::HttpResponse {
        debug!("{:?} register request", Self::user_kind());
        
        match state.auth_service.register(
            &req.nickname,
            &req.username,
            &req.password,
            Self::user_kind(),
            req.email.clone(),
            req.phone.clone(),
        ).await {
            Ok(response) => ResponseResult::success_with_data(response).response(),
            Err(e) => {
                tracing::error!("Registration failed: {}", e);
                ResponseResult::<()>::fail_with_message(&format!("注册失败: {}", e)).response()
            }
        }
    }
    
    /// 登出
    async fn logout(
        state: web::Data<AppState>,
        request: HttpRequest,
    ) -> actix_web::HttpResponse {
        let token = match extract_token(&request) {
            Some(t) => t,
            None => return ResponseResult::<()>::fail_with_message("未提供认证令牌").response(),
        };
        
        match state.auth_service.logout(Self::user_kind(), &token).await {
            Ok(_) => ResponseResult::<()>::success_with_message("登出成功").response(),
            Err(e) => {
                tracing::error!("Logout failed: {}", e);
                ResponseResult::<()>::fail_with_message(&format!("登出失败: {}", e)).response()
            }
        }
    }
}

/// MFA 控制器 Trait
/// 
/// 提供统一的 MFA 管理功能（生成、激活、停用）
pub trait MfaController {
    /// 返回用户类型
    fn user_kind() -> UserKind;
    
    /// 生成 MFA QR 码
    async fn generate_mfa(
        state: web::Data<AppState>,
        request: HttpRequest,
    ) -> actix_web::HttpResponse {
        use lib_mfa::TotpGenerator;
        use serde::Serialize;
        
        #[derive(Clone, Default, Serialize)]
        struct MfaGenerate {
            secret: String,
            qr_code: String,
            uri: String,
        }
        
        let token = match extract_token(&request) {
            Some(t) => t,
            None => return ResponseResult::<()>::fail_with_message("请先登陆").response(),
        };
        
        let user = match state.user_service.current_user(&Self::user_kind(), &token).await {
            Ok(u) => u,
            Err(_) => return ResponseResult::<()>::fail_with_message("获取用户信息失败").response(),
        };
        
        let mfa_secret = match state.mfa_service.generate(user.id).await {
            Ok(s) => s,
            Err(_) => return ResponseResult::<()>::fail_with_message("生成MFA密钥失败").response(),
        };
        
        let mfa_config = state.mfa_config.clone();
        let mfa_generate = match TotpGenerator::new(mfa_config, &mfa_secret, &user.username) {
            Ok(x) => x,
            Err(e) => {
                return ResponseResult::<()>::fail_with_message(&format!("生成MFA失败: {e}")).response();
            }
        };
        
        let mfa_uri = mfa_generate.get_uri();
        let png_base64 = match mfa_generate.get_qr_png_base64() {
            Ok(png) => png,
            Err(_) => return ResponseResult::<()>::fail_with_message("生成MFA二维码失败").response(),
        };
        
        let model = MfaGenerate {
            secret: mfa_secret,
            qr_code: png_base64,
            uri: mfa_uri,
        };
        
        ResponseResult::success_with_data(model).response()
    }
    
    /// 激活 MFA
    async fn activate_mfa(
        state: web::Data<AppState>,
        request: HttpRequest,
    ) -> actix_web::HttpResponse {
        let token = match extract_token(&request) {
            Some(t) => t,
            None => return ResponseResult::<()>::fail_with_message("请先登陆").response(),
        };
        
        let user = match state.user_service.current_user(&Self::user_kind(), &token).await {
            Ok(u) => u,
            Err(_) => return ResponseResult::<()>::fail_with_message("获取用户信息失败").response(),
        };
        
        if let Err(e) = state.mfa_service.active(user.id).await {
            return ResponseResult::<()>::fail_with_message(&format!("激活MFA失败: {e}")).response();
        }
        
        ResponseResult::<()>::success().response()
    }
    
    /// 停用 MFA
    async fn deactivate_mfa(
        state: web::Data<AppState>,
        request: HttpRequest,
    ) -> actix_web::HttpResponse {
        let token = match extract_token(&request) {
            Some(t) => t,
            None => return ResponseResult::<()>::fail_with_message("请先登陆").response(),
        };
        
        let user = match state.user_service.current_user(&Self::user_kind(), &token).await {
            Ok(u) => u,
            Err(_) => return ResponseResult::<()>::fail_with_message("获取用户信息失败").response(),
        };
        
        if let Err(e) = state.mfa_service.delete(user.id).await {
            return ResponseResult::<()>::fail_with_message(&format!("关闭MFA失败: {e}")).response();
        }
        
        ResponseResult::<()>::success().response()
    }
}

/// Passkey 控制器 Trait
/// 
/// 提供统一的 Passkey 管理功能（注册、登录、停用）
pub trait PasskeyController {
    /// 返回用户类型
    fn user_kind() -> UserKind;
    
    /// 开始 Passkey 注册
    async fn passkey_register_begin(
        state: web::Data<AppState>,
        username: String,
    ) -> actix_web::HttpResponse {
        debug!("Passkey register begin for {:?}: {}", Self::user_kind(), username);
        
        let user = match state.user_service.get_by_username(Self::user_kind(), &username).await {
            Ok(Some(user)) => user,
            Ok(None) => return ResponseResult::<()>::fail_with_message("用户不存在").response(),
            Err(e) => {
                tracing::error!("Failed to get user: {:?}", e);
                return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
            }
        };
        
        match state.passkey_service.register_begin(
            &username,
            user.id,
            &user.nickname,
        ).await {
            Ok(challenge) => {
                debug!("Passkey registration challenge created");
                actix_web::HttpResponse::Ok()
                    .content_type("application/json")
                    .json(challenge)
            }
            Err(e) => {
                tracing::error!("Passkey register begin failed: {}", e);
                ResponseResult::<()>::fail_with_message(&e.to_string()).response()
            }
        }
    }
    
    /// 完成 Passkey 注册
    async fn passkey_register_complete(
        state: web::Data<AppState>,
        username: String,
        credential: RegisterPublicKeyCredential,
    ) -> actix_web::HttpResponse {
        debug!("Passkey register complete for {:?}: {}", Self::user_kind(), username);
        
        let user = match state.user_service.get_by_username(Self::user_kind(), &username).await {
            Ok(Some(user)) => user,
            Ok(None) => return ResponseResult::<()>::fail_with_message("用户不存在").response(),
            Err(e) => {
                tracing::error!("Failed to get user: {:?}", e);
                return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
            }
        };
        
        match state.passkey_service.register_complete(
            &username,
            user.id,
            credential,
        ).await {
            Ok(()) => ResponseResult::<()>::success().response(),
            Err(e) => {
                tracing::error!("Passkey register complete failed: {}", e);
                ResponseResult::<()>::fail_with_message(&e.to_string()).response()
            }
        }
    }
    
    /// 开始 Passkey 登录
    async fn passkey_login_begin(
        state: web::Data<AppState>,
        username: String,
    ) -> actix_web::HttpResponse {
        debug!("Passkey login begin for {:?}: {}", Self::user_kind(), username);
        
        let user = match state.user_service.get_by_username(Self::user_kind(), &username).await {
            Ok(Some(user)) => user,
            Ok(None) => return ResponseResult::<()>::fail_with_message("用户不存在").response(),
            Err(e) => {
                tracing::error!("Failed to get user: {:?}", e);
                return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
            }
        };
        
        match state.passkey_service.login_begin(
            &username,
            user.id,
        ).await {
            Ok(challenge) => {
                debug!("Passkey login challenge created");
                actix_web::HttpResponse::Ok().json(challenge)
            }
            Err(e) => {
                tracing::error!("Passkey login begin failed: {}", e);
                ResponseResult::<()>::fail_with_message(&e.to_string()).response()
            }
        }
    }
    
    /// 完成 Passkey 登录
    async fn passkey_login_complete(
        state: web::Data<AppState>,
        request: HttpRequest,
        username: String,
        credential: PublicKeyCredential,
    ) -> actix_web::HttpResponse {
        debug!("Passkey login complete for {:?}: {}", Self::user_kind(), username);
        
        let user = match state.user_service.get_by_username(Self::user_kind(), &username).await {
            Ok(Some(user)) => user,
            Ok(None) => return ResponseResult::<()>::fail_with_message("用户不存在").response(),
            Err(e) => {
                tracing::error!("Failed to get user: {:?}", e);
                return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
            }
        };
        
        match state.passkey_service.login_complete(
            &username,
            user.id,
            credential,
        ).await {
            Ok(()) => {
                let user_device = extract_device(&request);
                
                match state.auth_service.create_session(&user, user_device).await {
                    Ok(session) => {
                        let login_response = state.auth_service.session_to_login_response(session, &user);
                        ResponseResult::success_with_data(login_response).response()
                    }
                    Err(e) => {
                        tracing::error!("Failed to create session: {:?}", e);
                        ResponseResult::<()>::fail_with_message("生成令牌失败").response()
                    }
                }
            }
            Err(e) => {
                tracing::error!("Passkey login complete failed: {}", e);
                ResponseResult::<()>::fail_with_message(&e.to_string()).response()
            }
        }
    }
    
    /// 停用 Passkey
    async fn passkey_deactivate(
        state: web::Data<AppState>,
        request: HttpRequest,
    ) -> actix_web::HttpResponse {
        let token = match extract_token(&request) {
            Some(t) => t,
            None => return ResponseResult::<()>::fail_with_message("请先登陆").response(),
        };
        
        let user = match state.user_service.current_user(&Self::user_kind(), &token).await {
            Ok(u) => u,
            Err(_) => return ResponseResult::<()>::fail_with_message("获取用户信息失败").response(),
        };
        
        if let Err(e) = state.passkey_service.deactived(user.id).await {
            return ResponseResult::<()>::fail_with_message(&format!("关闭PassKey失败: {e}")).response();
        }
        
        ResponseResult::<()>::success().response()
    }
}

/// OTP 控制器 Trait
/// 
/// 提供统一的 OTP（一次性密码）功能
pub trait OtpController {
    /// 返回用户类型
    fn user_kind() -> UserKind;
    
    /// OTP 登录
    async fn otp_login(
        state: web::Data<AppState>,
        request: HttpRequest,
        contact: String,
        otp_code: String,
    ) -> actix_web::HttpResponse {
        debug!("{:?} OTP login request", Self::user_kind());
        
        let user_device = extract_device(&request);
        
        match state.auth_service.login_with_code(
            &contact,
            &otp_code,
            user_device,
            Self::user_kind(),
        ).await {
            Ok(response) => ResponseResult::success_with_data(response).response(),
            Err(e) => {
                tracing::error!("OTP login failed: {}", e);
                ResponseResult::<()>::fail_with_message(
                    &format!("OTP登录失败: {}", e)
                ).response()
            }
        }
    }
    
    /// 发送 OTP
    async fn send_otp(
        state: web::Data<AppState>,
        request: HttpRequest,
        email: Option<String>,
        phone: Option<String>,
    ) -> actix_web::HttpResponse {
        debug!("Send OTP request for {:?}", Self::user_kind());
        
        let user_device = extract_device(&request);
        
        match state.auth_service.send_otp(
            email,
            phone,
            user_device,
            Self::user_kind(),
        ).await {
            Ok(response) => ResponseResult::success_with_data(response).response(),
            Err(e) => {
                tracing::error!("Send OTP failed: {}", e);
                ResponseResult::<()>::fail_with_message(
                    &format!("发送验证码失败: {}", e)
                ).response()
            }
        }
    }
}
