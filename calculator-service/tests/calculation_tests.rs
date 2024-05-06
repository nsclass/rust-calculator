use assert_json_diff::assert_json_include;
use calculator_service::app_run;
use calculator_service::config::AppConfig;
use calculator_service::router::calculation::CalculationRequest;
use serde_json::json;

#[tokio::test]
async fn calculation_api_test() {
    let config = AppConfig {
        host: "0.0.0.0".to_string(),
        port: 3031,
        static_dir: "calculator-ui/dist".to_string(),
    };

    let serve = app_run(config.clone()).await.unwrap();

    tokio::spawn(async move {
        serve.await.unwrap();
    });
    let client = reqwest::Client::new();

    let payload = CalculationRequest {
        infix: "1 + 2 * (3 + 4) / 2".to_string(),
    };

    let response = client
        .post(&format!("http://localhost:{}/calculate", config.port))
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let res: serde_json::Value = response.json().await.expect("failed to get a response");

    assert_json_include!(
        actual: res,
        expected: json!({
            "trace": {
                "postfix": [ "1", "2", "3", "4", "+", "*", "2", "/", "+" ]
            }
        })
    )
}
