use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserId(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
struct UserInner {
    /// The id of user
    id: Option<UserId>,
    /// The email of user
    email: String,
    /// The hashed password of user
    password: String,
    /// The first name of user
    first_name: String,
    /// The last name of user
    last_name: String,
    /// The created date of user
    created_at: DateTime<Utc>,
    /// The last date of updated info about
    updated_at: Option<DateTime<Utc>>,
    /// The user as admin
    is_admin: bool,
}

#[allow(dead_code)]
pub struct User {
    inner: UserInner,
}

impl User {
    pub fn new_without_id(
        email: String,
        password: String,
        first_name: String,
        last_name: String,
        is_admin: bool,
    ) -> Self {
        let inner = UserInner::new_without_id(
            email, password, first_name, last_name, is_admin,
        );

        Self { inner }
    }
}

impl UserInner {
    fn new_without_id(
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
