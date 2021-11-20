use anyhow::Result;
use async_trait::async_trait;

use crate::domain::user::User;

/// Save User Port
///
/// Save the user from username or email, if not found the user returns an
/// error.
#[async_trait]
pub trait SaveUserPort {
    async fn save_user(&self, user: User) -> Result<User>;
}
