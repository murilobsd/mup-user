mod config;

use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use password::PasswordAdapter;
use persistence::user_persistence_adapter::UserPersitenceAdapter;
use rest::state::RestServerState;
use rest::UserRestServer;
use user_application::application::service::get_user_service::GetUserService;
use user_application::application::service::new_user_service::NewUserService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let database_url: String = config::get("database_url");

    // TODO: .await? change return error to anyhow::Result
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    // Adapters
    let user_persistence_adapter = UserPersitenceAdapter::new(pool);
    let password_adapter = PasswordAdapter::new();

    // Services
    // New User Service
    let new_user_service = NewUserService::new(
        Box::new(user_persistence_adapter.clone()),
        Box::new(user_persistence_adapter.clone()),
        Box::new(password_adapter),
    );

    // Get User Service
    let get_user_service =
        GetUserService::new(Box::new(user_persistence_adapter));

    let server_state = RestServerState::new(
        Arc::new(new_user_service),
        Arc::new(get_user_service),
    );

    let listen_address: String = config::get("listen_address");

    let rest_server = UserRestServer::new(server_state);
    rest_server.run(&listen_address).await
}
