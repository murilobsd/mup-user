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
    pub password: String,
    /// the first name of user
    pub first_name: String,
    /// the last name of user
    pub last_name: String,
    /// the created date of user
    pub created_at: DateTime<Utc>,
    /// the last date of updated info about
    pub updated_at: Option<DateTime<Utc>>,
    /// the user as admin
    pub is_admin: bool,
}

impl User {
    pub fn new_without_id(
        email: String,
        password: String,
        first_name: String,
        last_name: String,
        is_admin: bool,
    ) -> Self {
        Self {
            id: None,
            email,
            password,
            first_name,
            last_name,
            is_admin,
            created_at: Utc::now(),
            updated_at: None,
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
