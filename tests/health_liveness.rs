#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use rust_kata_002::observability;

    #[actix_rt::test]
    async fn test_health_liveness_get() {
        let mut app =
            test::init_service(App::new().configure(observability::endpoints::config)).await;

        let request = test::TestRequest::get()
            .uri("/health/liveness")
            .to_request();

        let response = test::call_service(&mut app, request).await;

        assert!(response.status().is_success());
    }
}
