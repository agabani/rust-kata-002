use actix_web::{middleware, App, HttpServer};
use rust_kata_002::crates_io::CratesIoClient;
use rust_kata_002::interfaces::CrateRegistry;
use rust_kata_002::{dependency_graph, observability, proxy};
use std::env;
use std::time::Instant;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application_start = Instant::now();

    dotenv::dotenv().ok();
    env_logger::init();

    let crate_registry =
        env::var("CRATE_REGISTRY_BASE_URL").unwrap_or_else(|_| "https://crates.io".to_owned());

    let host_address = env::var("HOST_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_owned());
    let host_port = env::var("HOST_PORT").unwrap_or_else(|_| "8080".to_owned());
    let host_socket = format!("{}:{}", host_address, host_port);

    HttpServer::new(move || {
        App::new()
            .wrap(observability::middleware::metric_middleware())
            .wrap(observability::middleware::logger_middleware())
            .wrap(middleware::NormalizePath::default())
            .data(application_start)
            .data::<Box<dyn CrateRegistry>>(Box::new(CratesIoClient::new(&crate_registry).unwrap()))
            .configure(observability::endpoints::config)
            .configure(dependency_graph::endpoints::config)
            .configure(proxy::endpoints::config)
    })
    .bind(host_socket)?
    .run()
    .await
}
