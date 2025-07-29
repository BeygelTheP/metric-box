use axum::extract::Path;

pub async fn create_tenant(Path(id): Path<String>) -> String {
    format!("✅Created tenant {id}").to_string()
}

pub async fn delete_tenant(Path(id): Path<String>) -> String {
    format!("❌Deleted tenant {id}").to_string()
}
