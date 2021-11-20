use crate::domain::user::{User, UserId};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait GetUserUseCase {
    async fn get_user(&self, user_id: UserId) -> Result<User>;
}
