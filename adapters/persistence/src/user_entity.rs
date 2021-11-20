use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};

#[derive(sqlx::FromRow, Debug)]
pub struct UserEntity {
    pub id: Option<Uuid>,
    pub active: bool,
    pub email: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub username: Option<String>,
    pub password: String,
    pub salt: String,
}
