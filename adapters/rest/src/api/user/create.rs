use crate::state::RestServerState;
use actix_web::{web, HttpResponse};

use user_application::application::port::incoming::new_user_use_case::NewUserCommand;

pub async fn create_user(state: web::Data<RestServerState>) -> HttpResponse {
    let command =
        NewUserCommand::new("neide@m0x.ru".to_string(), "123".to_string());
    match state.new_user_use_case.new_user(&command).await {
        Ok(user) => {
            println!("{:?}", user);
            HttpResponse::Created()
                .content_type("application/json")
                .body("")
        }
        _ => HttpResponse::BadRequest()
            .content_type("application/json")
            .body("{\"not\": \"found\"}"),
    }
}
