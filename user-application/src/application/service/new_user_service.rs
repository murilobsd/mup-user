use anyhow::{anyhow, Result};
use async_trait::async_trait;

use super::error::ServiceError;
use crate::application::port::incoming::hash_password_port::HashPasswordPort;
use crate::application::port::incoming::new_user_use_case::{
    NewUserCommand, NewUserUseCase,
};
use crate::application::port::outgoing::load_user_port::LoadUserPort;
use crate::application::port::outgoing::save_user_port::SaveUserPort;
use crate::domain::user::User;

#[allow(dead_code)]
pub struct NewUserService {
    load_user_port: Box<dyn LoadUserPort + Send + Sync>,
    save_user_port: Box<dyn SaveUserPort + Send + Sync>,
    hash_password_port: Box<dyn HashPasswordPort + Send + Sync>,
}

impl NewUserService {
    pub fn new(
        load_user_port: Box<dyn LoadUserPort + Send + Sync>,
        save_user_port: Box<dyn SaveUserPort + Send + Sync>,
        hash_password_port: Box<dyn HashPasswordPort + Send + Sync>,
    ) -> Self {
        Self {
            load_user_port,
            save_user_port,
            hash_password_port,
        }
    }
}

// TODO: remove returning new user
#[async_trait]
impl NewUserUseCase for NewUserService {
    async fn new_user(&self, command: &NewUserCommand) -> Result<User> {
        let user = self.load_user_port.load_user(&command.email).await?;

        match user {
            Some(_) => {
                let error = ServiceError::UserExist(command.email.clone());
                return Err(anyhow!(error));
            }
            None => {
                // generating salt and password hash
                let (salt, hash_password) = self
                    .hash_password_port
                    .hash_password(&command.password)
                    .await?;

                // new user
                let user = User::new_without_id(
                    command.email.clone(),
                    salt,
                    hash_password,
                );

                // save user
                let user = self.save_user_port.save_user(user).await?;

                Ok(user)
            }
        }
    }
}
