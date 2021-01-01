use actix_web::{web, App, HttpServer};
use rust_kata_002::health::health_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/health").configure(health_routes)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
