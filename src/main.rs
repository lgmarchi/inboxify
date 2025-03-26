use inboxify::run;

#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    tokio::task::spawn_blocking(|| {
        _ = run();
    })
    .await
}
