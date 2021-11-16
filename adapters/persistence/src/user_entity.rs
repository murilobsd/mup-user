use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};

#[derive(sqlx::FromRow)]
pub struct UserEntity {
    pub id: Option<Uuid>,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub created_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub active: bool,
    pub username: Option<String>,
}
