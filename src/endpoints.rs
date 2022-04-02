use actix_web::{get,Responder};

#[get("/")]
async fn news() -> impl Responder {
    format!("news")
}

#[get("/health")]
async fn health() -> impl Responder {
    format!("Endpoints: /health ")
}