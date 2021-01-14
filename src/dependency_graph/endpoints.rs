use crate::dependency_graph::models::{Edge, Node, QueryParams, QueryResult};
use crate::models::ErrorResponse;
use actix_web::error::QueryPayloadError;
use actix_web::web::QueryConfig;
use actix_web::{error, web, HttpRequest, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(query_config()).route("", web::get().to(query));
}

fn query_config() -> QueryConfig {
    web::QueryConfig::default().error_handler(|err: QueryPayloadError, _: &HttpRequest| {
        let err_message = match &err {
            QueryPayloadError::Deserialize(err) => err.to_string(),
        };

        error::InternalError::from_response(
            err,
            HttpResponse::BadRequest().json(ErrorResponse {
                code: "query".to_owned(),
                description: err_message,
            }),
        )
        .into()
    })
}

async fn query(web::Query(query_parameters): web::Query<QueryParams>) -> HttpResponse {
    HttpResponse::Ok().json(QueryResult {
        data: Some(vec![
            Node {
                name: query_parameters.name,
                version: query_parameters.version,
                edges: Some(vec![
                    Edge {
                        relationship: "dependency".to_string(),
                        node: Node {
                            name: "name".to_string(),
                            version: "version".to_string(),
                            edges: None,
                        },
                    },
                    Edge {
                        relationship: "dev-dependency".to_string(),
                        node: Node {
                            name: "name".to_string(),
                            version: "version".to_string(),
                            edges: None,
                        },
                    },
                ]),
            },
            Node {
                name: "name".to_owned(),
                version: "version".to_owned(),
                edges: Some(vec![
                    Edge {
                        relationship: "dependency".to_string(),
                        node: Node {
                            name: "name".to_string(),
                            version: "version".to_string(),
                            edges: None,
                        },
                    },
                    Edge {
                        relationship: "dev-dependency".to_string(),
                        node: Node {
                            name: "name".to_string(),
                            version: "version".to_string(),
                            edges: None,
                        },
                    },
                ]),
            },
        ]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;

    #[actix_rt::test]
    async fn test_query_ok() {
        let response = query(web::Query::from_query("name=name&version=version").unwrap()).await;

        assert_eq!(response.status(), http::StatusCode::OK);
    }
}
