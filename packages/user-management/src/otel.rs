use axum::Router;
use axum_otel_metrics::{HttpMetricsLayer, HttpMetricsLayerBuilder};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

pub trait AxumOtel {
    fn build_otel_layer(self) -> Self;
}

impl<S> AxumOtel for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn build_otel_layer(self) -> Self {
        let _ = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers().unwrap();

        let metrics: HttpMetricsLayer = HttpMetricsLayerBuilder::new().build();

        self.layer(metrics)
            .layer(OtelInResponseLayer)
            .layer(OtelAxumLayer::default())
    }
}