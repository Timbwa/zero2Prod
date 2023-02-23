use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Should read configuration file");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Should connect to Postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    println!(
        "Server is up and running at 127.0.0.1:{}",
        configuration.application_port
    );
    run(listener, connection_pool)?.await
}
