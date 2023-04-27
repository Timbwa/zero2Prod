use env_logger::Env;
use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // `init` calls `set_logge` and falls back to printing all logs at info-level or
    // above if RUST_LOG environment variable has not been set
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    let configuration = get_configuration().expect("Should read configuration file");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Should connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
