mod query;

use crate::models::ErrorResponse;
use actix_web::error::QueryPayloadError;
use actix_web::web::QueryConfig;
use actix_web::{error, web, HttpRequest, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(query_config())
        .route("/", web::get().to(query::query));
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
