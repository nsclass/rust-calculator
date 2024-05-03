use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use calculator_engine::{calculate_str, CalculationTraceDetails};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculationRequest {
    pub infix: String,
}

pub async fn calculation(Json(request): Json<CalculationRequest>) -> impl IntoResponse {
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
