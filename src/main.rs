use actix_web::{middleware::Logger, App, HttpServer};

mod routes;
mod util;

use routes::{hello::hello, ping::ping};
use util::config::init_logger;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    init_logger();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%r \n in:%{Header}i out:%{Header}o"))
            .service(hello)
            .service(ping)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
