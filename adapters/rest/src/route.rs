use crate::api::user::{create_user, get_user};
use actix_web::web;

async fn health_check() -> &'static str {
    "pong\r\n"
}

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg.route("/health", web::get().to(health_check));
    cfg.route("/users", web::post().to(create_user));
    cfg.route("/users/<user_id>", web::get().to(get_user))
}
