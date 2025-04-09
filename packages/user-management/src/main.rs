use crate::db::AppState;
use crate::otel::AxumOtel;
use axum::{Router, serve};
use tokio::net::TcpListener;
use tracing::{info, info_span};

mod db;
mod otel;

#[tokio::main]
async fn main() {
    let app = Router::new().build_otel_layer();

    let span = info_span!("Server Start").entered();

    let addr = format!("0.0.0.0:{}", env!("PORT"));
    let app_state = AppState::setup(env!("DB_URL")).await.unwrap();

    let listener = TcpListener::bind(&addr).await.unwrap();

    let app = app
        .with_state(app_state)
        .route("/health", axum::routing::get(|| async { "Health Check" }))
        .into_make_service();

    info!("Listening on {}", addr);

    span.exit();

    serve(listener, app).await.unwrap();
}
