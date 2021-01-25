use rust_kata_002::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host_address = std::env::var("HOST_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_owned());
    let host_port = std::env::var("HOST_PORT").unwrap_or_else(|_| "8080".to_owned());
    let host_socket = format!("{}:{}", host_address, host_port);

    let listener = std::net::TcpListener::bind(&host_socket)
        .expect(&format!("Failed to bind to {}.", host_socket));

    run(listener)?.await
}
