use anyhow::Result;
use sqlx::postgres::PgPool;

#[derive(sqlx::FromRow)]
pub struct UserEntity {
    pub email: String,
}

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
        email_username: String,
    ) -> Result<Option<UserEntity>> {
        let result = sqlx::query_as::<_, UserEntity>(
            r#"
                SELECT email::TEXT
                FROM users
                WHERE email = $1::TEXT::CITEXT OR username = $2
            "#,
        )
        .bind(email_username.clone())
        .bind(email_username)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(result) => Ok(Some(UserEntity {
                email: result.email,
            })),
            None => Ok(None),
        }
    }
}
