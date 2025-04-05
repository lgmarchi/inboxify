use std::net::TcpListener;

use inboxify::{configuration::get_configuration, startup::run};
use sqlx::{Connection, PgConnection};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration =
        get_configuration().expect("Failed to read configuration.");
    let Ok(connection) =
        PgConnection::connect(&configuration.database.connection_string())
            .await
    else {
        panic!("Failed to connect to database.");
    };

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let tcp_listener = TcpListener::bind(address)?;

    run(tcp_listener, connection)?.await
}
