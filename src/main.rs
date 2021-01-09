use actix_web::{middleware, web, App, HttpServer};
use rust_kata_002::dependency_graph::dependency_graph_routes;
use rust_kata_002::health::health_routes;
use rust_kata_002::observability::endpoints::config as metrics_routes;
use rust_kata_002::observability::middleware::ObservabilityMetrics;
use std::time::Instant;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application_start = Instant::now();

    dotenv::dotenv().ok();
    env_logger::init();

    let host_address = std::env::var("HOST_ADDRESS").unwrap_or("0.0.0.0".to_owned());
    let host_port = std::env::var("HOST_PORT").unwrap_or("8080".to_owned());
    let host_socket = format!("{}:{}", host_address, host_port);

    HttpServer::new(move || {
        App::new()
            .wrap(
                ObservabilityMetrics::default()
                    .exclude("/metrics/")
                    .exclude_regex("/health/.*"),
            )
            .wrap(
                middleware::Logger::default()
                    .exclude("/metrics/")
                    .exclude_regex("/health/.*"),
            )
            .wrap(middleware::NormalizePath::default())
            .data(application_start)
            .service(web::scope("/metrics").configure(metrics_routes))
            .service(web::scope("/health").configure(health_routes))
            .service(web::scope("/dependency-graph").configure(dependency_graph_routes))
    })
    .bind(host_socket)?
    .run()
    .await
}
