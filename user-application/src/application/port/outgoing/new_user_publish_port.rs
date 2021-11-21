use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait NewUserPublishPort {
    async fn new_user_publish(&self, email: &str) -> Result<()>;
}
