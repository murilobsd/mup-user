mod api;
mod init;
mod route;
mod state;

use actix_web::{middleware, App, HttpServer};

pub struct UserRestServer {}

impl Default for UserRestServer {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRestServer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self, listen_address: &str) -> std::io::Result<()> {
        let state = state::initialize();

        println!("Listening to requests at {}...", listen_address);

        HttpServer::new(move || {
            App::new()
                .data(state.clone())
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
