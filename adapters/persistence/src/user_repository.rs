use anyhow::Result;
use sqlx::postgres::PgPool;

use super::user_entity::UserEntity;

#[derive(Clone, Debug)]
pub(crate) struct UserRepository {
    pool: PgPool,
}

#[allow(dead_code, unused_variables)]
impl UserRepository {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub(crate) async fn find_by_email_username(
        &self,
        email_username: &str,
    ) -> Result<Option<UserEntity>> {
        let result = sqlx::query!(
        //let result = sqlx::query_as::<_, UserEntity>(
            r#"
                SELECT id, email::TEXT, salt, password, created_at, confirmed_at, updated_at, active, username
                FROM users
                WHERE email = $1::TEXT::CITEXT OR username = $2
            "#,
            email_username,
            email_username
        )
        // .bind(email_username)
        // .bind(email_username)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(result) => Ok(Some(UserEntity {
                id: Some(result.id),
                email: result.email.unwrap(),
                salt: result.salt,
                password: result.password,
                created_at: result.created_at,
                confirmed_at: result.confirmed_at,
                updated_at: result.updated_at,
                active: result.active.unwrap(),
                username: result.username,
            })),
            None => Ok(None),
        }
    }

    pub(crate) async fn save(
        &self,
        user_entity: UserEntity,
    ) -> Result<UserEntity> {
        println!("{:?}", user_entity);
        let entity = sqlx::query!(
            r#"
                INSERT INTO users 
                    (email, salt, password)
                VALUES 
                    ($1::TEXT::CITEXT, $2, $3)
                RETURNING 
                    id, email::TEXT, salt, password, created_at, confirmed_at, updated_at, active, username
            "#,
            user_entity.email,
            user_entity.salt,
            user_entity.password
        )
        .fetch_one(&self.pool)
        .await?;

        let entity = UserEntity {
            id: Some(entity.id),
            email: entity.email.unwrap(),
            salt: entity.salt,
            password: entity.password,
            created_at: entity.created_at,
            confirmed_at: entity.confirmed_at,
            updated_at: entity.updated_at,
            active: entity.active.unwrap(),
            username: entity.username,
        };

        Ok(entity)
    }
}
