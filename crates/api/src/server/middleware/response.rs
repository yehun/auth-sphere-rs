use actix_http::{HttpMessage, ResponseHead};
use std::{
    future::{ready, Future, Ready},
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use actix_web::{
    body::{BodySize, MessageBody},
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web::{Bytes, BytesMut},
    Error
};
use pin_project_lite::pin_project;
use tracing::{debug, span, Level};

pub struct Logging;

impl<S: 'static, B> Transform<S, ServiceRequest> for Logging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BodyLogger<B>>;
    type Error = Error;
    type Transform = LoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddleware { service }))
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = ServiceResponse<BodyLogger<B>>;
    type Error = Error;
    type Future = WrapperStream<S, B>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        WrapperStream {
            future: self.service.call(request),
            _t: PhantomData,
        }
    }
}

pin_project! {
    pub struct WrapperStream<S, B>
    where
        B: MessageBody,
        S: Service<ServiceRequest>,
    {
        #[pin]
        future: S::Future,
        _t: PhantomData<(B,)>,
    }
}

impl<S, B> Future for WrapperStream<S, B>
where
    B: MessageBody,
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Output = Result<ServiceResponse<BodyLogger<B>>, Error>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let future: <<S as Service<ServiceRequest>>::Future as Future>::Output =
            futures_util::ready!(self.project().future.poll(ctx));
        match future {
            Ok(service_response) => {
                let http_request = service_response.request();
                let trace_id = {
                    let binding = http_request.extensions();
                    binding.get::<String>().unwrap_or(&"".to_string()).clone()
                };
                Poll::Ready(Ok(service_response.map_body(
                    move |_head: &mut ResponseHead, body: B| BodyLogger::from((trace_id, body)),
                )))
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

pin_project! {
    pub struct BodyLogger<B> {
        trace_id: String,
        #[pin]
        body: B,
        body_accum: BytesMut,
    }

    impl<B> PinnedDrop for BodyLogger<B> {
        fn drop(this: Pin<&mut Self>) {
            if this.body_accum.is_empty() {
                return
            }
            let trace_id = this.trace_id.clone();
            let span = span!(Level::DEBUG, "request", trace_id);
            let _enter = span.enter();
            let body_text = String::from_utf8_lossy(this.body_accum.as_ref());
            // let body = if body_text.len() > 2000 {
            //     body_text[..2000].trim()
            // } else {
            //     body_text.trim()
            // };
            let body = body_text.trim();
            debug!("response body: {}", body);
        }
    }
}

impl<B> From<(String, B)> for BodyLogger<B> {
    fn from(value: (String, B)) -> Self {
        let (trace_id, body) = value;
        Self {
            trace_id,
            body,
            body_accum: BytesMut::new(),
        }
    }
}

impl<B: MessageBody> MessageBody for BodyLogger<B> {
    type Error = B::Error;

    fn size(&self) -> BodySize {
        self.body.size()
    }

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let this = self.project();

        match this.body.poll_next(cx) {
            Poll::Ready(Some(Ok(chunk))) => {
                this.body_accum.extend_from_slice(&chunk);
                Poll::Ready(Some(Ok(chunk)))
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
