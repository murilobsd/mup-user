use std::sync::Arc;

use user_application::application::port::incoming::get_user_use_case::GetUserUseCase;
use user_application::application::port::incoming::new_user_use_case::NewUserUseCase;

#[derive(Clone)]
pub struct RestServerState {
    pub new_user_use_case: Arc<dyn NewUserUseCase + Send + Sync>,
    pub get_user_use_case: Arc<dyn GetUserUseCase + Send + Sync>,
}

impl RestServerState {
    pub fn new(
        new_user_use_case: Arc<dyn NewUserUseCase + Send + Sync>,
        get_user_use_case: Arc<dyn GetUserUseCase + Send + Sync>,
    ) -> Self {
        Self {
            new_user_use_case,
            get_user_use_case,
        }
    }
}
