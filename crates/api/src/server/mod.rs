use anyhow::Result;
use actix_web::{error, web, App, HttpServer};
use actix_web::dev::Server;
use actix_web::middleware::from_fn;
use actix_web_validator::{Error, FormConfig, JsonConfig, PathConfig, QsQueryConfig, QueryConfig};
use tracing::debug;
use model::response::base::result::ResponseResult;
use crate::config::AppState;

mod model;
pub(crate) mod service;
mod controller;
mod middleware;
pub(crate) mod config;

pub async fn init(state: AppState) -> Result<Server> {
    // let addr = "0.0.0.0:8080";
    let addr = crate::config::get_application().server.get_addr();
    debug!("start http://{}", addr);

    let create_app = move || {
        let cors_config = config::cors::get_config();
        let trace_config = config::tracing::get_config();

        App::new()
            .app_data(FormConfig::default().error_handler(error_handler))
            .app_data(PathConfig::default().error_handler(error_handler))
            .app_data(JsonConfig::default().error_handler(error_handler))
            .app_data(QueryConfig::default().error_handler(error_handler))
            .app_data(QsQueryConfig::default().error_handler(error_handler))
            .app_data(web::Data::new(state.clone()))
            .wrap(from_fn(middleware::header::header_middleware))
            .wrap(middleware::response::Logging)
            .wrap(middleware::request::Logging)
            // .wrap(from_fn(middleware::logging::logging_middleware))
            // .wrap(from_fn(middleware::token::token_middleware))
            .wrap(trace_config)
            // .wrap(TracingLogger::default())
            .wrap(cors_config)
            .configure(controller::init)
    };

    Ok(HttpServer::new(create_app)
        .bind(addr)?
        .run())
}


fn error_handler(err: Error, _request: &actix_web::HttpRequest) -> actix_web::Error {
    let result: ResponseResult<String> = match &err {
        Error::Validate(error) => ResponseResult::from(error),
        e => ResponseResult::fail_with_error(&*e)
    };
    debug!("error message: {}", err);
    error::InternalError::from_response(err, result.response()).into()
}