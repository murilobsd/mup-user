use anyhow::Result;
use async_trait::async_trait;

use crate::application::port::incoming::get_user_use_case::GetUserUseCase;
use crate::application::port::outgoing::get_user_port::GetUserPort;
use crate::domain::user::{User, UserId};

#[allow(dead_code)]
pub struct GetUserService {
    get_user_port: Box<dyn GetUserPort + Send + Sync>,
}

impl GetUserService {
    pub fn new(get_user_port: Box<dyn GetUserPort + Send + Sync>) -> Self {
        Self { get_user_port }
    }
}

#[async_trait]
impl GetUserUseCase for GetUserService {
    async fn get_user(&self, _user_id: UserId) -> Result<User> {
        unimplemented!();
    }
}
