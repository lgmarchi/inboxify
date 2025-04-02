use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// Form already verify if data from URL will deserialize correctly on FormData struct
pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
