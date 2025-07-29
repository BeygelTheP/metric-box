use axum::response::{IntoResponse, Json};
use serde_json::{json, Value};
use tracing::info;

pub async fn events(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    info!("request received");
    Json(payload)
}

pub async fn metrics() -> String {
    "metrics\n".to_string()
}

pub async fn events_batch() -> String {
    "events_batch\n".to_string()
}

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "healthy", "service": "metricbox-event-ingestion"}))
}
