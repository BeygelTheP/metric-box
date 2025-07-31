use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use serde_json::{json, Value};
use tracing::info;

use crate::{database::clickhouse_events, AppContext};

pub async fn events(
    Extension(app_context): Extension<Arc<AppContext>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    info!("Hit events endpoint with payload {}", payload);
    let client = Arc::clone(&(app_context));
    match clickhouse_events::events_handler(&client.public_clickhouse_client, payload).await {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            eprintln!("Failed to add event. {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
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
