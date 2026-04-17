use std::fs;
use actix_web::{web, Error, HttpResponse};
use actix_files::Files as Fs;
use actix_http::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use tracing::debug;

mod home;
mod member;
mod community;
mod platform;


pub(super) fn handler_response(req: ServiceRequest, message: &str) -> Result<ServiceResponse<BoxBody>, Error> {
    let resp = HttpResponse::Unauthorized().json(serde_json::json!({
            "code": 401,
            "message": message,
            "data": null
        }));
    let (http_req, _) = req.into_parts();
    Ok(ServiceResponse::new(http_req, resp).map_into_boxed_body())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    // API 路由
    // cfg.service(
    //     web::scope("")
    //         .route("/", web::get().to(home::index))
    //         .route("/assets/{filename:.*}", web::get().to(home::assets))
    // );
    cfg.route("/", web::get().to(home::index));
    cfg.route("/assets/{filename:.*}", web::get().to(home::assets));
    
    // 初始化认证路由
    member::init(cfg);
    community::init(cfg);
    platform::init(cfg);
}
