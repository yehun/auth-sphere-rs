use std::time::Instant;
use actix_http::{HttpMessage, StatusCode};
use actix_http::header::HeaderMap;
use actix_web::{
    Error,
    body::{MessageBody},
    dev::{self, ServiceResponse},
    middleware::{Next},
    web::self
};
use tracing::{debug, span, Level};
use crate::server::middleware::common;
use crate::server::middleware::common::ToMap;

/*
actix_web::middleware::from_fn(middleware::logging::logging_middleware)
*/
#[allow(dead_code)]
pub async fn logging_middleware(
    mut request: dev::ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let trace_id = {
        let binding = request.extensions();
        binding.get::<String>().unwrap().clone()
    };
    let span = span!(Level::DEBUG, "model", trace_id);
    let _enter = span.enter();
    let now = Instant::now();
    let path = request.path().to_string();
    debug!("request path: {path}");
    let headers = request.headers().clone();
    if !headers.is_empty() {
        debug!("request headers: {:?}", headers.to_map());
    }
    let query_params = request.query_string().to_string();
    if !query_params.is_empty() {
        debug!("request query: {query_params}");
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
    let response = next.call(request).await?;

    debug!("response time: {:?}", now.elapsed());
    let status: StatusCode = response.status();
    debug!( "response status code: {}", status.as_u16());
    let headers: HeaderMap = response.headers().clone();
    if !headers.is_empty() {
        debug!("response headers: {:?}", headers.to_map());
    }
    let (http_request, http_response) = response.into_parts();
    // if !is_ws {
    //     let (res, body) = http_response.into_parts();
    //     let body = body::to_bytes(body).await.ok().unwrap();
    //     let body_text = String::from_utf8_lossy(&body);
    //     debug!("response body: {}", body_text);
    //     let new_body = web::Bytes::from(body_text.to_string());
    //     let response = res.set_body(new_body);
    //     http_response = response;
    // }
    let res = ServiceResponse::new(http_request, http_response);
    Ok(res)
}
