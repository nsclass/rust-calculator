use axum::routing::post;
use axum::serve::Serve;
use axum::Router;

pub mod router;
use router::calculation::calculation;

pub mod config;

use crate::config::AppConfig;

pub async fn app_run(config: AppConfig) -> eyre::Result<Serve<Router, Router>, std::io::Error> {
    let router = Router::new().route("/calculate", post(calculation));
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;
    let serve = axum::serve(listener, router);
    Ok(serve)
}
