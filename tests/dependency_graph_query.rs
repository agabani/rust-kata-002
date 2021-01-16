#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use async_trait::async_trait;
    use mockall::predicate::*;
    use mockall::*;
    use rust_kata_002::dependency_graph;
    use rust_kata_002::dependency_graph::models::{Edge, Node, QueryResult};
    use rust_kata_002::errors::RustKataResult;
    use rust_kata_002::interfaces::get_crate_dependencies::DependencyResponse;
    use rust_kata_002::interfaces::{get_crate, get_crate_dependencies, CrateRegistry};
    use rust_kata_002::models::ErrorResponse;

    #[actix_rt::test]
    async fn test_dependency_graph_query_bad_request_name() {
        let mut app =
            test::init_service(App::new().configure(dependency_graph::endpoints::config)).await;

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
        let mut app =
            test::init_service(App::new().configure(dependency_graph::endpoints::config)).await;

        let request = test::TestRequest::get()
            .uri("/dependency-graph?name=name")
            .to_request();

        let response = test::call_service(&mut app, request).await;

        assert!(response.status().is_client_error());

        let result: ErrorResponse = test::read_body_json(response).await;

        assert_eq!(result.code, "query");
        assert_eq!(result.description, "missing field `version`");
    }

    #[actix_rt::test]
    async fn test_query_ok() {
        let mut mock = MockBar::new();

        mock.expect_get_crate_dependencies()
            .times(1)
            .returning(|_, _| {
                RustKataResult::Ok(get_crate_dependencies::Response {
                    dependencies: vec![
                        DependencyResponse {
                            id: 1,
                            version_id: 1,
                            crate_id: "crate-a".to_string(),
                            req: "1.0.1".to_string(),
                            optional: false,
                            default_features: false,
                            features: None,
                            target: None,
                            kind: "dev".to_string(),
                            downloads: 0,
                        },
                        DependencyResponse {
                            id: 2,
                            version_id: 2,
                            crate_id: "crate-b".to_string(),
                            req: "1.0.2".to_string(),
                            optional: false,
                            default_features: false,
                            features: None,
                            target: None,
                            kind: "normal".to_string(),
                            downloads: 0,
                        },
                    ],
                })
            });

        let mut app = test::init_service(
            App::new()
                .configure(dependency_graph::endpoints::config)
                .data::<Box<dyn CrateRegistry>>(Box::new(mock)),
        )
        .await;

        let request = test::TestRequest::get()
            .uri("/dependency-graph?name=name&version=version")
            .to_request();

        let response = test::call_service(&mut app, request).await;

        assert!(response.status().is_success());

        let result: QueryResult = test::read_body_json(response).await;

        assert_eq!(
            result,
            QueryResult {
                data: Some(vec![Node {
                    name: "name".to_string(),
                    version: "version".to_string(),
                    edges: Some(vec![
                        Edge {
                            relationship: "dev".to_string(),
                            node: Node {
                                name: "crate-a".to_string(),
                                version: "1.0.1".to_string(),
                                edges: None
                            }
                        },
                        Edge {
                            relationship: "normal".to_string(),
                            node: Node {
                                name: "crate-b".to_string(),
                                version: "1.0.2".to_string(),
                                edges: None
                            }
                        }
                    ])
                }])
            }
        );
    }

    mock! {
        pub Bar {}

        #[async_trait]
        impl CrateRegistry for Bar {
            async fn get_crate(&self, crate_name: &str) -> RustKataResult<get_crate::Response>;
            async fn get_crate_dependencies(&self, crate_name: &str, crate_version: &str) -> RustKataResult<get_crate_dependencies::Response>;
        }
    }
}
