use actix_web::{
    dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new().route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}
