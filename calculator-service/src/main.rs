mod app_config;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use color_eyre::Result;
use serde::de::Unexpected::Option;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::app_config::AppConfig;
use calculator_engine::{calculate_str, CalculationTraceDetails};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculationRequest {
    pub infix: String,
}

async fn calculation(Json(request): Json<CalculationRequest>) -> impl IntoResponse {
    let result = calculate_str(&request.infix, true);
    let json_res = match result {
        Ok((val, trace_details)) => {
            json!({
                "status": "Success".to_string(),
                "result": val,
                "trace": trace_details.unwrap()
            })
        }
        Err(err) => json!({
             "status": format!("Failed to calculate: {}", err),
        }),
    };

    (StatusCode::OK, Json(json_res))
}

#[tokio::main]
async fn main() -> Result<()> {
    // build our application with a single route
    let router = Router::new().route("/calculate", post(calculation));

    let conf = AppConfig::from_env()?;
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", conf.host, conf.port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
