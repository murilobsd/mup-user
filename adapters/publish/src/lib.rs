use anyhow::Result;
use async_trait::async_trait;
use user_application::application::port::outgoing::new_user_publish_port::NewUserPublishPort;

#[derive(Default, Debug, Clone)]
pub struct UserPublisherAdapater {}

impl UserPublisherAdapater {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl NewUserPublishPort for UserPublisherAdapater {
    async fn new_user_publish(&self, _email: &str) -> Result<()> {
        unimplemented!();
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
