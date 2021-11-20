use crate::state::RestServerState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::Serialize;
use user_application::domain::user::{User, UserId};

#[derive(Serialize)]
pub struct UserResp {
    email: String,
}

impl From<User> for UserResp {
    fn from(u: User) -> Self {
        Self { email: u.email }
    }
}

pub async fn get_user(
    req: HttpRequest,
    state: web::Data<RestServerState>,
) -> HttpResponse {
    let user_id = req.match_info().get("user_id").unwrap_or("World");
    let user_id = UserId(user_id.to_string());

    match state.get_user_use_case.get_user(user_id).await {
        Ok(u) => {
            let user_resp = UserResp::from(u);
            HttpResponse::Ok()
                .content_type("application/json")
                .json(user_resp)
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!("{{\"error\": \"{}\"}}", e.to_string())),
    }
}
