use crate::state::RestServerState;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn get_user(
    req: HttpRequest,
    _state: web::Data<RestServerState>,
) -> HttpResponse {
    let _user_id = req.match_info().get("user_id").unwrap_or("World");
    HttpResponse::Ok().content_type("application/json").body("")
    // match state.new_user_use_case.new_user(&command).await {
    //     Ok(_) => HttpResponse::Created()
    //         .content_type("application/json")
    //         .body(""),
    //     Err(e) => HttpResponse::BadRequest()
    //         .content_type("application/json")
    //         .body(format!("{{\"error\": \"{}\"}}", e.to_string())),
    // }
}
