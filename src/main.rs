use std::net::TcpListener;

use inboxify::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration =
        get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let tcp_listener = TcpListener::bind(address)?;

    run(tcp_listener)?.await
}
