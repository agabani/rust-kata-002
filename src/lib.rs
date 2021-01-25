#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

mod crates_io;
mod dependency_graph;
mod errors;
mod interfaces;
mod observability;
mod proxy;

use crate::crates_io::CratesIoClient;
use crate::interfaces::crate_registry::CrateRegistry;
use actix_web::dev::Server;
use actix_web::{middleware, App, HttpServer};
use std::env;
use std::time::Instant;

pub fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    let application_start = Instant::now();

    dotenv::dotenv().ok();
    env_logger::init();

    let crate_registry =
        env::var("CRATE_REGISTRY_BASE_URL").unwrap_or_else(|_| "https://crates.io".to_owned());

    let host_base_path = env::var("HOST_BASE_PATH").unwrap_or_else(|_| "".to_owned());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(observability::middleware::metric_middleware())
            .wrap(observability::middleware::logger_middleware())
            .wrap(middleware::NormalizePath::default())
            .data(application_start)
            .data::<Box<dyn CrateRegistry>>(Box::new(CratesIoClient::new(&crate_registry).unwrap()))
            .configure(observability::endpoints::config)
            .configure(|config| dependency_graph::endpoints::config(config, &host_base_path))
            .configure(|config| proxy::endpoints::config(config, &host_base_path))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
