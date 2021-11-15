use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::PgPool;

use user_application::application::port::outgoing::load_user_port::LoadUserPort;
use user_application::domain::user::User;

use crate::user_repository::UserRepository;

#[derive(Clone, Debug)]
pub struct UserPersitenceAdapter {
    user_repository: UserRepository,
}

impl UserPersitenceAdapter {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_repository: UserRepository::new(pool),
        }
    }
}

#[async_trait]
impl LoadUserPort for UserPersitenceAdapter {
    async fn load_user(&self, email_username: String) -> Result<Option<User>> {
        log::debug!("try to load user: {}", email_username);

        let user_entity = self
            .user_repository
            .find_by_email_username(email_username)
            .await?;
        match user_entity {
            Some(user) => {
                let user = User::new_without_id(
                    user.email,
                    "salt".to_string(),
                    "12345678".to_string(),
                );
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
