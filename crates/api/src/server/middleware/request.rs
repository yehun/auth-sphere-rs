use std::{
    future::{ready, Ready},
    rc::Rc,
};
use std::time::Instant;

use actix_http::HttpMessage;
use actix_http::header::HeaderMap;
use actix_web::{
    dev::{
        self, Service,
        ServiceRequest,
        ServiceResponse,
        Transform
    }, web, Error
};
use actix_web::http::StatusCode;
use futures_util::future::LocalBoxFuture;
use tracing::{debug, Level, span};
use crate::server::middleware::common;
use crate::server::middleware::common::ToMap;

pub struct Logging;

impl<S: 'static, B> Transform<S, ServiceRequest> for Logging
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct LoggingMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut request: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        Box::pin(async move {
            let trace_id = {
                let binding = request.extensions();
                binding.get::<String>().unwrap_or(&"".to_string()).clone()
            };
            let span = span!(Level::DEBUG, "request", trace_id);
            let _enter = span.enter();
            let now = Instant::now();
            let ip = request.peer_addr()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_else(|| "未知".to_string());
            debug!("request ip: {ip}");
            let path = request.path().to_string();
            debug!("request path: {path}");
            let headers = request.headers().clone();
            if !headers.is_empty() {
                debug!("request header:{:?}", headers.to_map());
            }
            let query_params = request.query_string();
            if !query_params.is_empty() {
                debug!("request query: {query_params} ");
            }
            let is_websocket = headers.get("upgrade")
                .map(|v| v == "websocket")
                .unwrap_or(false);
            if !is_websocket {
                let body = request.extract::<web::Bytes>().await?;
                if !body.is_empty() {
                    let body_text = String::from_utf8_lossy(&body);
                    debug!("request body: {}", body_text.to_string());
                }
                request.set_payload(common::bytes_to_payload(body));
            }
            let response = svc.call(request).await?;
            debug!("response time: {:?}", now.elapsed());
            let status: StatusCode = response.status();
            debug!( "response status code: {}", status.as_u16());
            let headers: HeaderMap = response.headers().clone();
            if !headers.is_empty() {
                debug!("response headers: {:?}", headers);
            }
            Ok(response)
        })
    }
}
