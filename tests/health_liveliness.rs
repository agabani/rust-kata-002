#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use rust_kata_002::health;

    #[actix_rt::test]
    async fn test_health_liveliness_get() {
        let mut app = test::init_service(
            App::new().service(web::scope("/health").configure(health::endpoints::config)),
        )
        .await;

        let request = test::TestRequest::get()
            .uri("/health/liveliness/")
            .to_request();

        let response = test::call_service(&mut app, request).await;

        assert!(response.status().is_success());
    }
}
