use crate::dependency_graph::models::{Edge, Node, QueryParams, QueryResult};
use actix_web::{web, HttpResponse};

pub(crate) async fn query(query_parameters: web::Query<QueryParams>) -> HttpResponse {
    let name = match &query_parameters.name {
        None => return HttpResponse::BadRequest().finish(),
        Some(name) => name.to_owned(),
    };
    let version = match &query_parameters.version {
        None => return HttpResponse::BadRequest().finish(),
        Some(version) => version.to_owned(),
    };

    HttpResponse::Ok().json(QueryResult {
        data: Some(vec![
            Node {
                name: name.to_owned(),
                version: version.to_owned(),
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
