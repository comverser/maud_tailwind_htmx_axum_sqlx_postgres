use std::time::Duration;

use axum::{body::Body, extract::{ConnectInfo, Request}, http::Response};
use tower::{ServiceBuilder, layer::util::{Identity, Stack}};
use tower_http::{
    classify::{ServerErrorsAsFailures, ServerErrorsFailureClass, SharedClassifier},
    trace::{DefaultOnBodyChunk, DefaultOnEos, TraceLayer},
};
use tracing::Span;

type HttpTraceLayer = ServiceBuilder<
    Stack<
        TraceLayer<
            SharedClassifier<ServerErrorsAsFailures>,
            fn(&Request<Body>) -> Span,
            fn(&Request<Body>, &Span),
            fn(&Response<Body>, Duration, &Span),
            DefaultOnBodyChunk,
            DefaultOnEos,
            fn(ServerErrorsFailureClass, Duration, &Span),
        >,
        Identity,
    >,
>;

pub fn create_http_trace_layer() -> HttpTraceLayer {
    ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(make_span as fn(&Request<Body>) -> Span)
            .on_request(on_request as fn(&Request<Body>, &Span))
            .on_response(on_response as fn(&Response<Body>, Duration, &Span))
            .on_failure(on_failure as fn(ServerErrorsFailureClass, Duration, &Span)),
    )
}

fn on_request(request: &Request<Body>, _: &Span) {
    let client_ip = request
        .extensions()
        .get::<ConnectInfo<std::net::SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip().to_string())
        .unwrap_or("unknown".to_string());

    tracing::info!(
        "-> Request started: client {} method {} path {}",
        client_ip,
        request.method(),
        request.uri().path()
    );
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span) {
    tracing::info!(
        "<- Response generated: status {} in {:?}",
        response.status(),
        latency
    );
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("-x- Request failed: {:?} after {:?}", error, latency);
}

fn make_span(_: &Request<Body>) -> Span {
    tracing::info_span!("http-request")
}
