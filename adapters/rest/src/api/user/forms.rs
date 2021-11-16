use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewUserForm {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 15))]
    pub password: String,
}
