use actix_web::web;

mod liveliness;
mod readiness;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/liveliness", web::get().to(liveliness::get))
        .route("/readiness", web::get().to(readiness::get));
}
