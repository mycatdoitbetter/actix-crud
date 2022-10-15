
pub mod handlers;
pub mod model;
pub mod schema;
pub mod services;

use actix_web::web;

use self::handlers::*;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(create)
        .service(read)
        .service(update)
        .service(delete);
}
