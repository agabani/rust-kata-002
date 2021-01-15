use crate::dependency_graph::models::{Edge, Node, QueryParams, QueryResult};
use crate::models::ErrorResponse;
use crate::traits::CrateRegistry;
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
