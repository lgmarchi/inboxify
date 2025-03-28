use std::net::TcpListener;

use inboxify::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:0")
        .expect("Not possible to bind provided address.");

    run(tcp_listener)?.await
}
