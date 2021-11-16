use anyhow::Result;
use argon2::Config;
use async_trait::async_trait;
use rand::Rng;

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
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        let hash_password =
            argon2::hash_encoded(text_password.as_bytes(), &salt, &config)?;

        let salt = String::from_utf8_lossy(&salt).into_owned();

        Ok((salt, hash_password))
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
