use super::forms::NewUserForm;
use crate::state::RestServerState;
use actix_web::{web, HttpResponse};

use actix_web_validator::Json;
use user_application::application::port::incoming::new_user_use_case::NewUserCommand;

pub async fn create_user(
    form: Json<NewUserForm>,
    state: web::Data<RestServerState>,
) -> HttpResponse {
    let command =
        NewUserCommand::new(form.email.to_string(), form.password.to_string());

    match state.new_user_use_case.new_user(&command).await {
        Ok(_) => HttpResponse::Created()
            .content_type("application/json")
            .body(""),
        Err(e) => HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!("{{\"error\": \"{}\"}}", e.to_string())),
    }
}
