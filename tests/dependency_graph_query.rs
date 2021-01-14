#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use rust_kata_002::dependency_graph;
    use rust_kata_002::models::ErrorResponse;

    #[actix_rt::test]
    async fn test_dependency_graph_query_bad_request_name() {
        let mut app = test::init_service(App::new().service(
            web::scope("/dependency-graph").configure(dependency_graph::endpoints::config),
        ))
        .await;

        let request = test::TestRequest::get()
            .uri("/dependency-graph?version=version")
            .to_request();

        let response = test::call_service(&mut app, request).await;

        assert!(response.status().is_client_error());

        let result: ErrorResponse = test::read_body_json(response).await;

        assert_eq!(result.code, "query");
        assert_eq!(result.description, "missing field `name`");
    }

    #[actix_rt::test]
    async fn test_dependency_graph_query_bad_request_version() {
        let mut app = test::init_service(App::new().service(
            web::scope("/dependency-graph").configure(dependency_graph::endpoints::config),
        ))
        .await;

        let request = test::TestRequest::get()
            .uri("/dependency-graph?name=name")
            .to_request();

        let response = test::call_service(&mut app, request).await;

        assert!(response.status().is_client_error());

        let result: ErrorResponse = test::read_body_json(response).await;

        assert_eq!(result.code, "query");
        assert_eq!(result.description, "missing field `version`");
    }
}
