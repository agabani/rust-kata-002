use actix_web::HttpResponse;

pub(crate) async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;

    #[actix_rt::test]
    async fn test_get_ok() {
        let response = get().await;

        assert_eq!(response.status(), http::StatusCode::OK);
    }
}
