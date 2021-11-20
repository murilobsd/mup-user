use anyhow::Result;
use async_trait::async_trait;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use user_application::application::port::incoming::hash_password_port::HashPasswordPort;

#[derive(Debug, Default, Clone)]
pub struct PasswordAdapter {}

impl PasswordAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl HashPasswordPort for PasswordAdapter {
    async fn hash_password(
        &self,
        text_password: &str,
    ) -> Result<(String, String)> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash_password = argon2
            .hash_password(text_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        Ok((salt.as_str().to_string(), hash_password))
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
