use actix_web::{App, HttpServer};

mod handlers;
mod scrapers;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
