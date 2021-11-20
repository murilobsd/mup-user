use super::user_entity::UserEntity;
use sqlx::types::Uuid;
use user_application::domain::user::{User, UserId};

#[derive(Clone, Debug)]
pub struct UserMapper {}

impl UserMapper {
    pub fn new() -> Self {
        Self {}
    }

    pub fn map_to_domain(&self, user_entity: UserEntity) -> User {
        User {
            id: Some(UserId(user_entity.id.unwrap().to_string())),
            email: user_entity.email,
            password: user_entity.password,
            salt: user_entity.salt,
            created_at: user_entity.created_at,
            confirmed_at: user_entity.confirmed_at,
            updated_at: user_entity.updated_at,
            active: user_entity.active,
            username: user_entity.username,
        }
    }

    pub fn map_to_entity(&self, user: User) -> UserEntity {
        let user_id = user.id.map(|id| id.0);
        let entity_id = user_id.map(|id| Uuid::parse_str(&id).unwrap());

        UserEntity {
            id: entity_id,
            email: user.email,
            salt: user.salt,
            password: user.password,
            created_at: user.created_at,
            confirmed_at: user.confirmed_at,
            updated_at: user.updated_at,
            active: user.active,
            username: user.username,
        }
    }
}

impl Default for UserMapper {
    fn default() -> Self {
        Self::new()
    }
}
