use clickhouse::Client;
use tracing::info;

pub fn create_admin_client() -> Client {
    create_client("read_write", &get_env_var("CLICKHOUSE_PUB_USER_PWD"))
}

pub fn create_public_client() -> Client {
    create_client("admin", &get_env_var("CLICKHOUSE_ADMIN_USER_PWD"))
}
fn get_env_var(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|err| {
        panic!("{} {}", var, err);
    })
}

fn create_client(user: &str, password: &str) -> Client {
    info!("Creationg clickhouse client for user {}.", user);
    Client::default()
        .with_url(get_env_var("CLICKHOUSE_URL"))
        .with_user(user)
        .with_password(password)
        .with_database(get_env_var("CLICKHOUSE_DB"))
}
