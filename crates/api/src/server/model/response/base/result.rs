use std::error::Error;
use actix_http::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ResponseResult<T: Clone> {
    #[serde(rename = "code")]
    code: i32,

    #[serde(rename = "message")]
    message: String,

    #[serde(rename = "data")]
    data: Option<T>,
}

impl<T> Responder for ResponseResult<T>
where
    T: Clone + Default + Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        match serde_json::to_string(&self) {
            Ok(val) => {
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(val)
            }
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
}

impl<T> ResponseResult<T>
where
    T: Clone + Default + Serialize
{
    pub fn json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn response(&self) -> HttpResponse<BoxBody> {
        self.response_json()
    }

    pub fn response_json(&self) -> HttpResponse<BoxBody> {
        match self.json() {
            Ok(val) => {
                // HttpResponse::Ok().json(val)
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(val)
            },
            Err(e) => HttpResponse::ServiceUnavailable().body(e.to_string()),
        }
    }
}

#[allow(dead_code)]
impl<T> ResponseResult<T>
where
    T: Clone
{
    pub fn success() -> ResponseResult<T> {
        Self::success_with_message("success")
    }

    pub fn success_with_message(message: &str) -> ResponseResult<T> {
        Self {
            code: 0,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn success_with_data(data: T) -> ResponseResult<T> {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn fail() -> ResponseResult<T> {
        Self::fail_with_message("fail")
    }

    pub fn fail_with_message(message: &str) -> ResponseResult<T> {
        Self {
            code: -1,
            message: message.to_string(),
            data: Default::default(),
        }
    }

    pub fn fail_with_error<E: Error>(error: E) -> ResponseResult<T> {
        Self::fail_with_message(&error.to_string())
    }

    pub fn valid_with_message(message: &str) -> ResponseResult<T> {
        Self::valid_with_code_message(-9, message)
    }

    pub fn valid_with_error<E: Error>(error: E) -> ResponseResult<T> {
        Self::valid_with_message(&error.to_string())
    }

    pub fn valid_with_code_message(code: i32, message: &str) -> ResponseResult<T> {
        Self {
            code,
            message: message.to_string(),
            data: Default::default(),
        }
    }

    pub fn error() -> ResponseResult<T> {
        Self::error_with_message("error")
    }

    pub fn error_with_error<E: Error>(error: E) -> ResponseResult<T> {
        Self::error_with_message(&error.to_string())
    }

    pub fn error_with_message(message: &str) -> ResponseResult<T> {
        Self {
            code: -999,
            message: message.to_string(),
            data: Default::default(),
        }
    }
}
