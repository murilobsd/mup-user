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

    pub(crate) async fn save(&self, user: UserEntity) -> Result<UserEntity> {
        unimplemented!();
    }
}
