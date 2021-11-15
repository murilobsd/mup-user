use std::sync::Arc;

use user_application::application::port::incoming::new_user_use_case::NewUserUseCase;

#[derive(Clone)]
pub struct RestServerState {
    pub new_user_use_case: Arc<dyn NewUserUseCase + Send + Sync>,
}

impl RestServerState {
    pub fn new(
        new_user_use_case: Arc<dyn NewUserUseCase + Send + Sync>,
    ) -> Self {
        Self { new_user_use_case }
    }
}
