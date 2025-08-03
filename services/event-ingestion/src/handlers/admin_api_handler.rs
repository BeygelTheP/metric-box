use crate::{database::clickhouse_tenants, AppContext};
use axum::extract::Path;
use axum::{http::StatusCode, response::IntoResponse, Extension};
use std::sync::Arc;

pub async fn create_tenant(
    Extension(app_context): Extension<Arc<AppContext>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let client = Arc::clone(&(app_context));
    match clickhouse_tenants::add_tenant_table(&client.public_clickhouse_client, &id).await {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            eprintln!("Failed to add event. {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_tenant(
    Extension(app_context): Extension<Arc<AppContext>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let client = Arc::clone(&(app_context));
    match clickhouse_tenants::delete_tenant_table(&client.public_clickhouse_client, &id).await {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            eprintln!("Failed to add event. {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
