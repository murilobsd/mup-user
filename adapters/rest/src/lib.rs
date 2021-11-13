use actix_web::{get, middleware, App, HttpServer};

pub struct UserRestServer {}

#[get("/health")]
async fn health_check() -> &'static str {
    "pong\r\n"
}

impl Default for UserRestServer {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRestServer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self, addr: &str, port: u16) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .wrap(
                    middleware::DefaultHeaders::new()
                        .header("X-Version", "0.1"),
                )
                .wrap(middleware::Compress::default())
                .wrap(middleware::Logger::default())
                .service(health_check)
        })
        .bind((addr, port))?
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
