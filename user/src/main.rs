use rest::UserRestServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let rest_server = UserRestServer::new();
    rest_server.run("0.0.0.0", 8080_u16).await
}
