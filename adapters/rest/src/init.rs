use crate::route::setup_routes;
use actix_web::web::ServiceConfig;

pub fn initialize(cfg: &mut ServiceConfig) {
    setup_routes(cfg);
}
