use crate::domain::user::User;
use anyhow::Result;
use async_trait::async_trait;

pub struct NewUserCommand {
    /// The email of user
    pub email: String,
    /// The hashed password of user
    pub password: String,
}

impl NewUserCommand {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

#[async_trait]
pub trait NewUserUseCase {
    async fn new_user(&self, command: &NewUserCommand) -> Result<User>;
}
