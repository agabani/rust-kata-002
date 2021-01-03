mod query;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(query::query));
}
