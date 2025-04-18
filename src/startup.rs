use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgConnection;

use crate::routes::{health_check, subscribe};

pub fn run(
    listener: TcpListener,
    connection: PgConnection,
) -> Result<Server, std::io::Error> {
    // Wrap the connection in a web::Data, it uses Arc internally
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
