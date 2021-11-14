use crate::application::port::incoming::new_user_use_case::{
    NewUserCommand, NewUserUseCase,
};
use crate::domain::user::User;
use anyhow::Result;
use async_trait::async_trait;

pub struct NewUserService {}

#[allow(clippy::new_without_default)]
impl NewUserService {
    pub fn new() -> Self {
        Self {}
    }
}

#[allow(unused_variables)]
#[async_trait]
impl NewUserUseCase for NewUserService {
    async fn new_user(&self, command: &NewUserCommand) -> Result<User> {
        unimplemented!()
    }
}
