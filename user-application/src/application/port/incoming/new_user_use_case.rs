use crate::domain::user::User;
use anyhow::Result;
use async_trait::async_trait;

pub struct NewUserCommand {
    /// The email of user
    pub email: String,
    /// The hashed password of user
    pub password: String,
    /// The first name of user
    pub first_name: String,
    /// The last name of user
    pub last_name: String,
}

impl NewUserCommand {
    pub fn new_without_id(
        email: String,
        password: String,
        first_name: String,
        last_name: String,
    ) -> Self {
        Self {
            email,
            password,
            first_name,
            last_name,
        }
    }
}

#[async_trait]
pub trait NewUserUseCase {
    async fn new_user(&self, command: &NewUserCommand) -> Result<User>;
}
