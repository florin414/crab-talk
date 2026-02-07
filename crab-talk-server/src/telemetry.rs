use http::Request as HttpRequest;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing::{info_span, Span};

use opentelemetry::{global, KeyValue};
use opentelemetry::trace::{TracerProvider};
use opentelemetry_sdk::{
    trace::{SdkTracerProvider},
    Resource,
};
use opentelemetry_otlp::SpanExporter;

pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let exporter = SpanExporter::builder()
        .with_tonic()
        .build()?;

    let resource = Resource::builder_empty()
        .with_attributes([
            KeyValue::new("service.name", "crab-talk-server"),
            KeyValue::new("env", "dev"),
            KeyValue::new("version", "0.1.0"),
        ])
        .build();

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    global::set_tracer_provider(provider.clone());

    let tracer = provider.tracer("crab-talk-server");

    let telemetry_layer = OpenTelemetryLayer::new(tracer);

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_span_events(fmt::format::FmtSpan::FULL);

    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(telemetry_layer)
        .init();

    Ok(())
}

pub fn trace_fn_adapter(req: &HttpRequest<()>) -> Span {
    let grpc_path = req
        .headers()
        .get(":path")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    info_span!("grpc_request", grpc.method = %grpc_path)
}

pub fn get_span(req: &HttpRequest<()>) -> Span {
    req.extensions().get::<Span>().cloned().unwrap_or_else(Span::current)
}
