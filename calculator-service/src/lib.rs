use axum::routing::post;
use axum::serve::Serve;
use axum::Router;
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::fmt::format;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod router;
use router::calculation::calculation;

pub mod config;

use crate::config::AppConfig;

pub async fn app_run(config: AppConfig) -> eyre::Result<Serve<Router, Router>, std::io::Error> {
    let formatting_layer = BunyanFormattingLayer::new("calculator".to_string(), std::io::stdout);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "calculator_service=debug,tower_http=trace".into()),
        )
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();

    let index_file = format!("{}/index.html", config.static_dir);

    let serve_dir = ServeDir::new(config.static_dir).not_found_service(ServeFile::new(index_file));

    let router = Router::new()
        .route("/calculate", post(calculation))
        .route_service("/", serve_dir);

    let router = router.layer(TraceLayer::new_for_http());

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let serve = axum::serve(listener, router);
    Ok(serve)
}
