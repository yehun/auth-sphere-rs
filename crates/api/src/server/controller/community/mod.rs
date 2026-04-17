mod user;
mod otp;
mod login;
mod passkey;

use actix_http::body::{BoxBody, MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Next};
use actix_web::{web, Error};
use auth_sphere_db::table::user::UserKind;
use crate::config::AppState;
use crate::server::controller::handler_response;
use crate::server::middleware::Authorization;

async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>
) -> Result<ServiceResponse<BoxBody>, Error> {
    let Some(auth) = Authorization::get(&req) else {
        return handler_response(req, "未提供认证令牌");
    };
    let Some(state) = req.app_data::<AppState>() else {
        return handler_response(req, "获取应用状态失败");
    };
    let Ok(user) = state.user_service.current_user(UserKind::Community, &auth.0).await else {
        return handler_response(req, "错误的认证令牌");
    };
    if user.user_type != UserKind::Community {
        return handler_response(req, "用户类型错误");
    }
    let res = next.call(req).await?;
    Ok(res.map_into_boxed_body())
}


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/community")
            .route("/register", web::post().to(login::register))
            .route("/login", web::post().to(login::login_with_password))
            .route("/otp/send", web::post().to(otp::send))
            .route("/otp/login", web::post().to(otp::login))
            // Passkey 路由
            .route("/passkey/register/begin", web::post().to(passkey::register_begin))
            .route("/passkey/register/complete", web::post().to(passkey::register_complete))
            .route("/passkey/login/begin", web::post().to(passkey::login_begin))
            .route("/passkey/login/complete", web::post().to(passkey::login_complete))
            .service(
                web::scope("")
                    .wrap(from_fn(auth_middleware))
                    .route("/logout", web::post().to(login::logout))
                    .route("/info", web::get().to(user::me))
            )
    );
}