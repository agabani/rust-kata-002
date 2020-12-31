use actix_web::web;

mod get;
mod liveliness;
mod readiness;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get::get))
        .route("/liveliness", web::get().to(liveliness::get))
        .route("/readiness", web::get().to(readiness::get));
}
