use crate::state::RestServerState;
use actix_web::{web, HttpResponse};

pub async fn create_user(_state: web::Data<RestServerState>) -> HttpResponse {
    HttpResponse::Created()
        .content_type("application/json")
        .body("")
}
