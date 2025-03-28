use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let port = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let tcp_listener = TcpListener::bind("127.0.0.1:0").expect(
        "Not possible 
        to bind provided address.",
    );

    let port = tcp_listener.local_addr().unwrap().port();

    // Port 0 make the OS look for
    // any available port to use
    let server = inboxify::run(tcp_listener).expect("Failed to bind address");

    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
