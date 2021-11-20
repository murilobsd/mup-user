use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::PgPool;

use user_application::application::port::outgoing::load_user_port::LoadUserPort;
use user_application::application::port::outgoing::save_user_port::SaveUserPort;
use user_application::domain::user::User;

use crate::user_mapper::UserMapper;
use crate::user_repository::UserRepository;

#[derive(Clone, Debug)]
pub struct UserPersitenceAdapter {
    user_repository: UserRepository,
    user_mapper: UserMapper,
}

impl UserPersitenceAdapter {
    pub fn new(pool: PgPool) -> Self {
        Self {
            user_repository: UserRepository::new(pool),
            user_mapper: UserMapper::new(),
        }
    }
}

#[async_trait]
impl LoadUserPort for UserPersitenceAdapter {
    async fn load_user(&self, email_username: &str) -> Result<Option<User>> {
        log::debug!("try to load user: {}", email_username);

        let user_entity = self
            .user_repository
            .find_by_email_username(email_username)
            .await?;

        match user_entity {
            Some(user) => {
                let user = self.user_mapper.map_to_domain(user);
                Ok(Some(user))
            }
            None => {
                log::debug!("not found {}", &email_username);
                Ok(None)
            }
        }
    }
}

#[async_trait]
impl SaveUserPort for UserPersitenceAdapter {
    async fn save_user(&self, _user: User) -> Result<User> {
        unimplemented!();
    }
}
