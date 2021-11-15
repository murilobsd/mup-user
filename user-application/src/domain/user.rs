use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserId(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct User {
    /// the id of user
    pub id: Option<UserId>,
    /// the email of user
    pub email: String,
    /// the hashed password of user
    pub username: Option<String>,
    /// the hashed password of user
    pub password: String,
    /// the salt password of user
    pub salt: String,
    /// the first name of user
    pub created_at: DateTime<Utc>,
    /// the last date of updated info about
    pub updated_at: Option<DateTime<Utc>>,
    /// the last date of updated info about
    pub confirmed_at: Option<DateTime<Utc>>,
    /// the user id active
    pub active: bool,
}

impl User {
    pub fn new_without_id(
        email: String,
        salt: String,
        password: String,
    ) -> Self {
        Self {
            id: None,
            email,
            username: None,
            salt,
            password,
            active: false,
            created_at: Utc::now(),
            updated_at: None,
            confirmed_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
