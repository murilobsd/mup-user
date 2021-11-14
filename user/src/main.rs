mod config;

use rest::UserRestServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let listen_address: String = config::get("listen_address");

    let rest_server = UserRestServer::new();
    rest_server.run(&listen_address).await
}
