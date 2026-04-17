use actix_http::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{web, Error, HttpResponse};

// mod home;
mod member;
mod community;
mod platform;
mod helper;
mod base;
mod page;

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
    member::init(cfg);
    community::init(cfg);
    platform::init(cfg);

    cfg.route("/", web::get().to(page::index));
    cfg.route("/assets/{filename:.*}", web::get().to(page::assets));
    // cfg.route("/", web::get().to(home::index));
    // cfg.route("/assets/{filename:.*}", web::get().to(home::assets));
}
