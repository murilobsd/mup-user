use anyhow::Result;
use async_trait::async_trait;

/// Hash password port
#[async_trait]
pub trait HashPasswordPort {
    async fn hash_password(
        &self,
        email_username: &str,
    ) -> Result<(String, String)>;
}
