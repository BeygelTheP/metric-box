use clickhouse::Client;
use serde::Serialize;
use tracing::info;

pub async fn events_handler(client: &Client, payload: serde_json::Value) -> anyhow::Result<()> {
    info!("Attempt to add {}.", payload);
    let mut inserter = client.insert("events")?;
    inserter.write(&Event { payload: payload }).await?;
    inserter.end().await?;
    info!("Success.");
    Ok(())
}

#[derive(Debug, Serialize, serde::Deserialize, clickhouse::Row)]
struct Event {
    
    #[serde(rename = "payload")]
    payload: serde_json::Value,
}
