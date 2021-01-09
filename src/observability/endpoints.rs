use actix_web::{web, HttpResponse};
use prometheus::{Encoder, TextEncoder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(get));
}

async fn get() -> HttpResponse {
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();

    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}
