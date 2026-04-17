use actix_http::HttpMessage;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use tracing::{debug, Level, Span};
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder};
use uuid::Uuid;

pub struct LoggerTraceBuilder;

impl RootSpanBuilder for LoggerTraceBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        let trace_id = Uuid::new_v4().to_string().replace("-", "");
        request.extensions_mut().insert(trace_id.clone());
        debug!("model initial trace_id is {trace_id}");
        // let root_span = RootSpanType::on_request_start(&request);
        // let root_span_wrapper = RootSpan::new(root_span.clone());
        // request.extensions_mut().insert(root_span_wrapper);
        // tracing_actix_web::root_span!(trace_id);
        // Span::none()
        tracing::span!(Level::TRACE, "trace_id", trace_id)

        // tracing_actix_web::root_span!(
        //     trace_id
        // )
    }

    fn on_request_end<B: MessageBody>(span: Span, response: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, response);
    }
}
