use actix_web::{web::Data, App, HttpServer};
use football_app_api::{self, utils::db::connect_db};



#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenvy::dotenv().ok();
    let db = connect_db().
        await.expect("Failed to connect to db");

    let db_data = Data::new(db);
    HttpServer::new( move || App::new().app_data(db_data.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
