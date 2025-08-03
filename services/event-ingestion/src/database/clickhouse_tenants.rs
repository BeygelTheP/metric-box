use anyhow::Ok;
use clickhouse::Client;
use tracing::info;

pub async fn add_tenant_table(client: &Client, tenant_id: &str) -> anyhow::Result<()> {
    let table_name = format!("events_tenant_{}", tenant_id);
    let copy_sql = format!(
        "CREATE TABLE metricbox.{} AS events_template",
        table_name
    );

    client.query(&copy_sql).execute().await?;
    info!("Created table: events_tenant_{} from template", table_name);
    Ok(())
}
pub async fn delete_tenant_table(client: &Client, tenant_id: &str) -> anyhow::Result<()> {
    let table_name = format!("events_tenant_{}", tenant_id);
    
    let drop_sql = format!("DROP TABLE IF EXISTS metricbox.{}", table_name);
    client.query(&drop_sql).execute().await?;
    
    info!("Deleted table: {}", table_name);
    Ok(())
}
