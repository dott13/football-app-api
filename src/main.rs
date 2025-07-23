use actix_web::{web::Data, App, HttpServer};

use crate::{scrapers::league::competition_scraper::get_competition, seeders::competition_seeder::seed_competition, utils::{client::build_client, db::connect_db}};

mod handlers;
mod scrapers;
mod models;
mod utils;
mod seeders;
mod entity;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenvy::dotenv().ok();
    let db = connect_db().
        await.expect("Failed to connect to db");

    let client = build_client()
        .expect("Failed to build client");
    println!("Client has been built");
    let comps = get_competition(&client, "https://www.transfermarkt.com/wettbewerbe/europa").await.expect("Failed to get the leagues");
    println!("Scraped {} competitions", comps.len());

    seed_competition(&db, comps)
        .await
        .expect("Failed to seed the competitions");
    println!("Seeded competitions into the database");

    let db_data = Data::new(db);
    HttpServer::new( move || App::new().app_data(db_data.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
