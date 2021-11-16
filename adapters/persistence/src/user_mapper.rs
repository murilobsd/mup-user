use super::user_entity::UserEntity;
use user_application::domain::user::{User, UserId};

#[derive(Clone, Debug)]
pub struct UserMapper {}

impl UserMapper {
    pub fn new() -> Self {
        Self {}
    }

    pub fn map_to_domain(&self, user_entity: UserEntity) -> User {
        let user = User {
            id: Some(UserId(user_entity.id.unwrap().to_string())),
            email: user_entity.email,
            password: user_entity.password,
            salt: user_entity.salt,
            created_at: user_entity.created_at,
            confirmed_at: user_entity.confirmed_at,
            updated_at: user_entity.updated_at,
            active: user_entity.active,
            username: user_entity.username,
        };
        user
    }
}

impl Default for UserMapper {
    fn default() -> Self {
        Self::new()
    }
}
