use crate::{health::models, health::models::Check};
use actix_web::{web, HttpResponse};
use chrono::{SecondsFormat, Utc};
use std::collections::HashMap;
use std::time::Instant;

pub(crate) async fn get(application_start: web::Data<Instant>) -> HttpResponse {
    let mut checks = HashMap::<String, Vec<Check>>::new();
    let uptime = checks.entry("uptime".to_owned()).or_insert(vec![]);

    uptime.push(Check {
        component_id: None,
        component_type: Some("system".to_owned()),
        observed_value: Some(application_start.elapsed().as_secs_f32().to_string()),
        observed_unit: Some("s".to_owned()),
        status: Some("pass".to_owned()),
        affected_endpoints: None,
        time: Some(Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)),
        output: None,
        links: None,
        additional_keys: None,
    });

    HttpResponse::Ok()
        .header("content-type", "application/health+json")
        .json(models::Health {
            status: "pass".to_owned(),
            version: Some(env!("CARGO_PKG_VERSION_MAJOR").to_owned()),
            release_id: Some(env!("CARGO_PKG_VERSION").to_owned()),
            notes: None,
            output: None,
            checks: Some(checks),
            links: None,
            service_id: None,
            description: Some("health of rust-kata-002 service".to_owned()),
        })
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
