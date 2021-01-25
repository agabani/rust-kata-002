#[cfg(test)]
mod tests {
    #[actix_rt::test]
    async fn test_health_liveness_get() {
        let address = spawn_app();

        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/health/liveness", address))
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(response.content_length(), Some(0));
    }

    pub fn spawn_app() -> String {
        let listener =
            std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");

        let port = listener.local_addr().unwrap().port();

        let server = rust_kata_002::run(listener).expect("Failed to bind address.");

        let _ = tokio::spawn(server);

        format!("http://127.0.0.1:{}", port)
    }
}
