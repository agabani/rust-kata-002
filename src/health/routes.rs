use actix_web::{get, web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(liveliness).service(readiness);
}

#[get("/liveliness")]
async fn liveliness() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/readiness")]
async fn readiness() -> impl Responder {
    HttpResponse::Ok()
}
