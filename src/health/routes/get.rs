use super::super::models;
use actix_web::HttpResponse;

pub(crate) async fn get() -> HttpResponse {
    HttpResponse::Ok()
        .header("content-type", "application/health+json")
        .json(models::Health {
            status: "pass".to_owned(),
            version: Some(env!("CARGO_PKG_VERSION_MAJOR").to_owned()),
            release_id: Some(env!("CARGO_PKG_VERSION").to_owned()),
            notes: None,
            output: None,
            checks: None,
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
        let response = get().await;

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
