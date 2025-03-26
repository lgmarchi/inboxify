use inboxify::run;

#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn our app");

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> Result<(), tokio::task::JoinError> {
    tokio::task::spawn_blocking(|| {
        _ = run();
    })
    .await
}
