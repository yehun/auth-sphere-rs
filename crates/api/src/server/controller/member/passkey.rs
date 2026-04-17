use actix_web::{web, Responder};
use actix_web_validator::Json;
use webauthn_rs::prelude::Url;
use auth_sphere_db::table::user::User;
use crate::config::AppState;
use crate::server::model::request::MfaLoginRequest;
use crate::server::model::response::base::result::ResponseResult;

pub async fn register_begin(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    body: Json<MfaLoginRequest>,
) -> impl Responder {
    // let user = state.user_store.get_or_create_user(&body.username).await?;
    // let (creation_challenge, user_id) = state.webauthn
    //     .start_passkey_registration(
    //         user.get_id(),
    //         &user.get_name(),
    //         &user.get_display_name(),
    //         None,
    //     )
    //     .map_err(|_| HttpError::InternalError)?;
    // 将挑战与用户 ID 存入 session 以便后续验证
    // state.session_store.insert(user_id, creation_challenge);
    // HttpResponse::Ok().json(creation_challenge)
    ResponseResult::<()>::fail_with_message("登录失败").response()
}

pub async fn register_complete(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    body: Json<MfaLoginRequest>,
) -> impl Responder {
    // // 从 session 中取出之前存储的挑战和用户信息
    // let (user_id, challenge) = state.session_store.get(&body.session_id)?;
    // let user = state.user_store.get_user(user_id)?;
    // // 完成注册验证
    // let credential = state.webauthn
    //     .finish_passkey_registration(&challenge, &body.credential, &user)
    //     .map_err(|_| HttpError::InvalidRequest)?;
    // // 将新凭证存储到数据库
    // state.credential_store.add_credential(&user, credential).await?;
    ResponseResult::<()>::fail_with_message("登录失败").response()
}


pub async fn login_begin(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    body: Json<MfaLoginRequest>,
) -> impl Responder {
    // let user = state.user_store.get_user_by_name(&body.username)?;
    // let allowed_credentials = state.credential_store.get_credentials(&user);
    // let (request_challenge, user_id) = state
    //     .webauthn
    //     .start_passkey_authentication(&allowed_credentials, &user)
    //     .map_err(|_| HttpError::InternalError)?;
    // state.session_store.insert(user_id, request_challenge);
    ResponseResult::<()>::fail_with_message("登录失败").response()
}

pub async fn login_complete(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
    body: Json<MfaLoginRequest>,
) -> impl Responder {
    // let (user_id, challenge) = state.session_store.get(&body.session_id)?;
    // let user = state.user_store.get_user(user_id)?;
    // let mut credentials = state.credential_store.get_credentials(&user);
    // // 完成认证验证
    // let auth_result = state
    //     .webauthn
    //     .finish_passkey_authentication(&challenge, &body.credential, &mut credentials, &user)
    //     .map_err(|_| HttpError::InvalidRequest)?;
    // // 更新凭证的签名计数器（重要！用于防重放攻击）
    // state.credential_store.update_credential_sign_count(auth_result).await?;
    // // 发放 JWT 或建立 Session
    // let token = create_jwt(&user);
    ResponseResult::<()>::fail_with_message("登录失败").response()
}


