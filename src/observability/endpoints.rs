use crate::observability::checkers::uptime_checker;
use crate::observability::models::Health;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use prometheus::{Encoder, TextEncoder};
use std::collections::HashMap;
use std::time::Instant;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/metrics").route("", web::get().to(get_metrics)));

    cfg.service(
        web::scope("/health")
            .route("", web::get().to(get))
            .route("/liveliness", web::get().to(probe))
            .route("/readiness", web::get().to(probe)),
    );
}

async fn get_metrics() -> HttpResponse {
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();

    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}

async fn get(application_start: web::Data<Instant>) -> HttpResponse {
    let now = Utc::now();

    let mut checks = HashMap::new();
    checks
        .entry("uptime".to_owned())
        .or_insert(vec![])
        .push(uptime_checker(&now, application_start.get_ref()));

    HttpResponse::Ok()
        .header("content-type", "application/health+json")
        .json(Health::envelope(checks))
}

async fn probe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;

    #[actix_rt::test]
    async fn test_get_ok() {
        let response = get(web::Data::new(Instant::now())).await;

        assert_eq!(response.status(), http::StatusCode::OK);

        let content_type_header = response
            .headers()
            .get("content-type")
            .expect("expected 'content-type' header")
            .to_str()
            .expect("expected header to only contain visible ASCII");
        assert_eq!(content_type_header, "application/health+json");
    }

    #[actix_rt::test]
    async fn probe_ok() {
        let response = probe().await;

        assert_eq!(response.status(), http::StatusCode::OK);
    }
}
