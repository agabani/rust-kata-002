use crate::dependency_graph::models::{Edge, Node, QueryParams, QueryResult};
use crate::interfaces::crate_registry::CrateRegistry;
use crate::interfaces::http;
use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig, host_base_path: &str) {
    let path = format!("{}/dependency-graph", host_base_path);

    cfg.service(
        web::scope(&path)
            .app_data(http::query_config())
            .route("", web::get().to(query)),
    );
}

async fn query(
    web::Query(query_parameters): web::Query<QueryParams>,
    crates_io_client: web::Data<Box<dyn CrateRegistry>>,
) -> HttpResponse {
    let result = crates_io_client
        .get_crate_dependencies(&query_parameters.name, &query_parameters.version)
        .await
        .unwrap();

    let edges = result
        .dependencies
        .iter()
        .map(|dependency| Edge {
            relationship: dependency.kind.to_owned(),
            node: Node {
                name: dependency.crate_id.to_owned(),
                version: dependency.req.to_owned(),
                edges: None,
            },
        })
        .collect();

    HttpResponse::Ok().json(QueryResult {
        data: Some(vec![Node {
            name: query_parameters.name.to_owned(),
            version: query_parameters.version.to_owned(),
            edges: Some(edges),
        }]),
    })
}
