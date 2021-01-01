use actix_web::{web, App, HttpServer};
use rust_kata_002::health::health_routes;
use std::time::Instant;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application_start = Instant::now();

    HttpServer::new(move || {
        App::new()
            .data(application_start)
            .service(web::scope("/health").configure(health_routes))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
