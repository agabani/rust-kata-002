use actix_web::error::QueryPayloadError;
use actix_web::web::QueryConfig;
use actix_web::{error, web, HttpRequest, HttpResponse};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "name")]
    pub code: String,
    #[serde(rename = "description")]
    pub description: String,
}

pub fn query_config() -> QueryConfig {
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
