use anyhow::Result;
use async_trait::async_trait;

use crate::application::port::incoming::new_user_use_case::{
    NewUserCommand, NewUserUseCase,
};
use crate::application::port::outgoing::load_user_port::LoadUserPort;
use crate::domain::user::User;

#[allow(dead_code)]
pub struct NewUserService {
    load_user_port: Box<dyn LoadUserPort + Send + Sync>,
}

impl NewUserService {
    pub fn new(load_user_port: Box<dyn LoadUserPort + Send + Sync>) -> Self {
        Self { load_user_port }
    }
}

#[allow(unused_variables)]
#[async_trait]
impl NewUserUseCase for NewUserService {
    async fn new_user(&self, command: &NewUserCommand) -> Result<User> {
        let user = self.load_user_port.load_user(command.email.clone()).await?;
        match user {
            Some(user) => Ok(user),
            None => {
                let user = User::new_without_id(
                    "deveria retornar um error".to_string(),
                    "salt".to_string(),
                    "12345678".to_string(),
                );
                Ok(user)
            }
        }
    }
}
