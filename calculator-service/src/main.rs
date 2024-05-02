use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Option;
use serde_json::json;

use calculator_engine::{calculate_str, CalculationTraceDetails};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculationRequest {
    pub infix: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CalculationResponse {
    pub status: String,
    pub result: f64,
    pub trace: CalculationTraceDetails
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
        })
    };

    (StatusCode::OK, Json(json_res))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/calculate", post(calculation));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // let input = "1 + 2";
    // let result = calculate_str(input, true);
    // match result {
    //     Ok((val, _trace_details)) => println!("{} is {}", input, val),
    //     Err(err) => println!("{}, error: {:?}", input, err),
    // }
}
