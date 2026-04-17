use actix_http::HttpMessage;
use actix_web::{web, Responder};
use serde::Serialize;
use auth_sphere_db::table::user::UserKind;
use lib_mfa::TotpGenerator;
use crate::config::AppState;
use crate::server::middleware::Authorization;
use crate::server::model::response::base::result::ResponseResult;

#[derive(Clone, Default, Serialize)]
pub struct MfaGenerate {
    pub secret: String,
    pub qr_code: String,
    pub uri: String,
}

pub async fn generate(
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
    let Ok(user) = state.user_service.current_user(UserKind::Platform, token).await else {
        return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
    };
    let Ok(mfa_secret) = state.mfa_service.generate(user.id).await else {
        return ResponseResult::<()>::fail_with_message("生成MFA密钥失败").response();
    };
    let mfa_config = state.mfa_config.clone();
    let mfa_generate = match TotpGenerator::new(mfa_config, &mfa_secret, &user.username) {
        Ok(x) => x,
        Err(e) => {
            return ResponseResult::<()>::fail_with_message(&format!("生成MFA失败: {e}")).response();
        }
    };
    let mfa_uri = mfa_generate.get_uri();
    let Ok(png_base64) = mfa_generate.get_qr_png_base64() else {
        return ResponseResult::<()>::fail_with_message("生成MFA二维码失败").response();
    };
    let model = MfaGenerate {
        secret: mfa_secret,
        qr_code: png_base64,
        uri: mfa_uri
    };
    ResponseResult::success_with_data(model).response()
}


pub async fn active(
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
    let Ok(user) = state.user_service.current_user(UserKind::Platform, token).await else {
        return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
    };
    if let Err(e) = state.mfa_service.active(user.id).await {
        return ResponseResult::<()>::fail_with_message(&format!("激活MFA失败: {e}")).response();
    }
    ResponseResult::<()>::success().response()
}


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
    let Ok(user) = state.user_service.current_user(UserKind::Platform, token).await else {
        return ResponseResult::<()>::fail_with_message("获取用户信息失败").response();
    };
    if let Err(e) = state.mfa_service.delete(user.id).await {
        return ResponseResult::<()>::fail_with_message(&format!("关闭MFA失败: {e}")).response();
    }
    ResponseResult::<()>::success().response()
}
