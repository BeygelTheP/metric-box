// services/event-ingestion/src/main.rs
use serde_json::json;
use std::convert::Infallible;
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber;
use warp::Filter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Starting MetricBox Event Ingestion service...");

    // Health endpoint
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&json!({
                "status": "healthy",
                "service": "metricbox-event-ingestion"
            }))
        });

    // Generate sample data endpoint
    let generate_sample = warp::path!("api" / "generate-sample")
        .and(warp::post())
        .map(|| {
            info!("📊 Generating sample data...");
            warp::reply::json(&json!({
                "status": "success",
                "message": "Sample data generation started",
                "events_generated": 1000
            }))
        });

    // Event ingestion endpoint
    let ingest_events = warp::path!("api" / "events")
        .and(warp::post())
        .and(warp::body::json())
        .map(|event: serde_json::Value| {
            info!("📥 Received event: {}", event);
            warp::reply::json(&json!({
                "status": "success",
                "message": "Event ingested"
            }))
        });

    let routes = health
        .or(generate_sample)
        .or(ingest_events)
        .with(warp::cors().allow_any_origin());

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    info!("📦 MetricBox Event Ingestion service listening on {}", addr);

    warp::serve(routes).run(addr).await;

    Ok(())
}