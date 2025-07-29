mod handlers;

use anyhow::Ok;
use axum::{
    routing::{delete, get, post},
    Router,
};
use handlers::{admin_api_handler, public_api_handler};
use std::env::{self};
use tracing::{info, Level};
use tracing_subscriber::{self};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("🚀 Starting MetricBox Event Ingestion service...");
    let ports = get_ports();
    tokio::try_join!(
        start_public_service(ports.public),
        start_admin_service(ports.admin)
    )?;
    Ok(())
}

fn get_port_from_env(port_name: &str) -> anyhow::Result<u16> {
    let port = env::var(&port_name)?.parse::<u16>()?;
    Ok(port)
}

async fn start_public_service(port: u16) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/api/events", post(public_api_handler::events))
        .route("/api/events-batch", post(public_api_handler::events_batch))
        .route("/health", get(public_api_handler::health))
        .route("/metrics", get(public_api_handler::metrics));

    let addr = format!("0.0.0.0:{port}");
    let listner = tokio::net::TcpListener::bind(&addr).await?;
    info!("👨‍💻 Event Ingestion service listening on {}", addr);
    axum::serve(listner, router).await?;
    Ok(())
}

async fn start_admin_service(port: u16) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/admin/tenant/{id}", post(admin_api_handler::create_tenant))
        .route(
            "/admin/tenant/{id}",
            delete(admin_api_handler::delete_tenant),
        );

    let addr = format!("0.0.0.0:{port}");
    let listner = tokio::net::TcpListener::bind(&addr).await?;
    info!("👮‍♀️ Admin Service  listening on {}", addr);
    axum::serve(listner, router).await?;
    Ok(())
}

fn get_ports() -> Ports {
    const PUBLIC_PORT_ENV_NAME: &str = "PUBLIC_PORT";
    const ADMIN_PORT_ENV_NAME: &str = "ADMIN_PORT";

    let public_port = get_port_from_env(PUBLIC_PORT_ENV_NAME).inspect_err(|err| {
        eprintln!(
            "❌ Failed to get {} env variable: {:#}",
            PUBLIC_PORT_ENV_NAME, err
        );
    });
    let admin_port = get_port_from_env(ADMIN_PORT_ENV_NAME).inspect_err(|err| {
        eprintln!(
            "❌ Failed to get {} env variable: {:#}",
            ADMIN_PORT_ENV_NAME, err
        );
    });
    if public_port.is_err() || admin_port.is_err() {
        panic!("❌ Could not read all required ports from environment.");
    }
    Ports {
        admin: admin_port.unwrap(),
        public: public_port.unwrap(),
    }
}
struct Ports {
    admin: u16,
    public: u16,
}
