use actix_web::{middleware::Logger, App, HttpServer};

mod routes;
mod util;

use routes::{hello::hello, ping::ping, solana::{get_balance, account, program_accounts}};
use util::config::init_logger;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    init_logger();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %r \n in:%{Header}i out:%{Header}o"))
            .service(hello)
            .service(ping)
            .service(get_balance)
            .service(account)
            .service(program_accounts)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
