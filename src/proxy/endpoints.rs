use crate::interfaces::CrateRegistry;
use crate::models::ErrorResponse;
use actix_web::error::QueryPayloadError;
use actix_web::web::QueryConfig;
use actix_web::{error, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/proxy")
            .app_data(query_config())
            .route("/crate", web::get().to(get_crate))
            .route("/crate_dependencies", web::get().to(get_crate_dependency)),
    );
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

async fn get_crate(
    web::Query(query_parameters): web::Query<GetCrateQueryParams>,
    crates_io_client: web::Data<Box<dyn CrateRegistry>>,
) -> HttpResponse {
    let response = crates_io_client
        .get_crate(&query_parameters.name)
        .await
        .unwrap();
    HttpResponse::Ok().json(response)
}

async fn get_crate_dependency(
    web::Query(query_parameters): web::Query<GetCrateDependenciesQueryParams>,
    crates_io_client: web::Data<Box<dyn CrateRegistry>>,
) -> HttpResponse {
    let response = crates_io_client
        .get_crate_dependencies(&query_parameters.name, &query_parameters.version)
        .await
        .unwrap();
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize, Serialize)]
pub(crate) struct GetCrateQueryParams {
    #[serde(rename = "name")]
    pub(crate) name: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct GetCrateDependenciesQueryParams {
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "version")]
    pub(crate) version: String,
}
