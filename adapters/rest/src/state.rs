use std::sync::Arc;
use user_application::application::port::incoming::new_user_use_case::NewUserUseCase;
use user_application::application::service::new_user_service::NewUserService;

#[derive(Clone)]
pub struct RestServerState {
    pub new_user_use_case: Arc<dyn NewUserUseCase + Send + Sync>,
}

pub fn initialize() -> RestServerState {
    let new_user_service = NewUserService::new();
    RestServerState {
        new_user_use_case: Arc::new(new_user_service),
    }
}
