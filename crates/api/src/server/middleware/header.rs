use actix_http::HttpMessage;
use actix_web::{
    Error,
    body::{MessageBody},
    dev::{self, ServiceResponse},
    middleware::{Next}
};
use tracing::{debug, span, Level};
use crate::server::middleware::{Authorization, DeviceType};


const DEVICE_KEY: &'static str = "X-DEVICE";
const AUTHORIZATION_KEY: &'static str = "X-AUTHORIZATION";


/*
actix_web::middleware::from_fn(middleware::header::header_middleware)
*/
#[allow(dead_code)]
pub async fn header_middleware(
    request: dev::ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let headers = request.headers().clone();
    {
        let x_device = headers.get(DEVICE_KEY)
            .and_then(|h| h.to_str().ok())
            .map(|x| x.trim())
            .map(|h| h.to_lowercase());
        let device = x_device
            .map(|x| DeviceType::from(x.as_str()))
            .unwrap_or(DeviceType::Unknown);
        debug!("request device: {:?}", device);
        request.extensions_mut().insert(device);
    }

    {
        let x_authorization = headers.get(AUTHORIZATION_KEY)
            .and_then(|h| h.to_str().ok())
            .map(|x| x.trim())
            .map(|h| h.to_lowercase());
        if let Some(x_authorization) = x_authorization {
            debug!("request authorization: {:?}", x_authorization);
            request.extensions_mut().insert(Authorization(x_authorization));
        }
    }

    let response = next.call(request).await?;
    let (http_request, http_response) = response.into_parts();
    let res = ServiceResponse::new(http_request, http_response);
    Ok(res)
}
