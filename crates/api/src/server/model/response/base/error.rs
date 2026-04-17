use actix_http::body::BoxBody;
use actix_web::{error, HttpResponse};
use crate::server::model::response::base::result::ResponseResult;

impl<T: Clone + Default> From<&validator::ValidationErrors> for ResponseResult<T> {
    fn from(error: &validator::ValidationErrors) -> ResponseResult<T> {
        let errors = error.field_errors().iter()
            .filter(|(_, errors)| !errors.is_empty())
            .map(|(_field, errors)| {
                // format!(
                //     "{} => {}",
                //     field.to_string(),
                //     errors.iter().map(|x| x.code.clone()).collect::<Vec<_>>().join(",")
                // )
                // errors.iter().map(|x| x.message.clone().unwrap().clone()).collect::<Vec<_>>().join(",")
                errors.first().unwrap().clone()
            })
            // .filter(|x| x.is_some())
            // .map(|x| x.unwrap())
            .collect::<Vec<validator::ValidationError>>();
        Self::valid_with_error(errors.first().unwrap())
    }

}

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum WebError {
    #[error("{0}")]
    Failed(String),
    #[error("validation error: {0}")]
    Valid(String),
    #[error("no auth error: {message:?}")]
    NoAuth { message: String },
    #[error("event error: {message:?}")]
    Logic { message: String },
    #[error("internal error")]
    Internal,
}


impl error::ResponseError for WebError {

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let result: ResponseResult<Option<()>> = match self {
            WebError::Failed(message) => {
                ResponseResult::<Option<()>>::fail_with_message(message)
            },
            WebError::Valid(message) => {
                let message = if message.is_empty() {
                    "Validation error"
                } else {
                    message
                };
                ResponseResult::<Option<()>>::valid_with_message(message)
            },
            WebError::NoAuth { message } => {
                ResponseResult::<Option<()>>::fail_with_message(message)
            },
            WebError::Logic { message} => {
                ResponseResult::<Option<()>>::fail_with_message(message)
            },
            _ => ResponseResult::<Option<()>>::error_with_message("系统错误")
        };
        result.response()
    }

}

impl From<error::Error> for WebError {
    fn from(err: error::Error) -> Self {
        WebError::Logic { message: err.to_string() }
    }
}

// impl From<SQLxError> for WebError {
//     fn from(err: SQLxError) -> Self {
//         MyError::DBError(err.to_string())
//     }
// }