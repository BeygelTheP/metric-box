use clickhouse::Client;
use tracing::info;

pub fn create_admin_client() -> Client {
    let conn_str = std::env::var("PUBLIC_CLICKHOUSE_CONNECTION").unwrap_or_else(|err| {
        panic!("{}", err);
    });
    create_client(&conn_str)
}

pub fn create_public_client() -> Client {
    let conn_str = std::env::var("ADMIN_CLICKHOUSE_CONNECTION").unwrap_or_else(|err| {
        panic!("{}", err);
    });
    create_client(&conn_str)
}

fn create_client(conn_str: &str) -> Client {
    info!("Creationg clickhouse client with {}.", conn_str);
    Client::default().with_url(conn_str)
}
