use anyhow::Result;
use async_trait::async_trait;

use crate::domain::user::User;

/// Load User Port
///
/// Load the user from username or email, if not found the user returns an
/// error.
#[async_trait]
pub trait LoadUserPort {
    async fn load_user(&self, email_username: String) -> Result<Option<User>>;
}
