use std::net::TcpListener;

fn spawn_app() -> String {
    let tcp_listener = TcpListener::bind("127.0.0.1:0").expect(
        "Not possible 
        to bind provided address.",
    );

    let port = tcp_listener.local_addr().unwrap().port();

    // Port 0 make the OS look for
    // any available port to use
    let server =
        inboxify::startup::run(tcp_listener).expect("Failed to bind address");

    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=lucas%20marchi&email=lucasmarchi%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app_address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "Missing the email"),
        ("email=ursula_le_guin%40gmail.com", "Missing the name"),
        ("", "Missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app_address))
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400, response.status().as_u16(),
        "The API did not fail with 400 Bad Request when the payload was {}.", error_message)
    }
}
