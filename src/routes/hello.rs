use actix_web::{get, web, Responder};

#[get("/hello/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    tracing::info!("Hello");
    format!("Hello {name}!")
}