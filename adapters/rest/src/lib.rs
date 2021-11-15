mod api;
mod init;
mod route;
pub mod state;

use actix_web::{middleware, web, App, HttpServer};

#[derive(Clone)]
pub struct UserRestServer {
    server_state: state::RestServerState,
}

impl UserRestServer {
    pub fn new(server_state: state::RestServerState) -> Self {
        Self { server_state }
    }

    pub async fn run(&self, listen_address: &str) -> std::io::Result<()> {
        let state = self.server_state.clone();
        let data = web::Data::new(state.clone());

        println!("Listening to requests at {}...", listen_address);

        HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .configure(init::initialize)
                .wrap(
                    middleware::DefaultHeaders::new()
                        .header("X-Version", "0.1"),
                )
                .wrap(middleware::Compress::default())
                .wrap(middleware::Logger::default())
        })
        .bind(listen_address)?
        .run()
        .await
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
