use crate::health::checkers::uptime_checker;
use crate::health::envelope::envelope;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use std::collections::HashMap;
use std::time::Instant;

pub(crate) async fn get(application_start: web::Data<Instant>) -> HttpResponse {
    let now = Utc::now();

    let mut checks = HashMap::new();
    checks
        .entry("uptime".to_owned())
        .or_insert(vec![])
        .push(uptime_checker(&now, application_start.get_ref()));

    HttpResponse::Ok()
        .header("content-type", "application/health+json")
        .json(envelope(checks))
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
}
