use actix_web::{web::Data, App, HttpServer};

use crate::{scrapers::league::league_teams_scraper::get_league_teams, utils::{client::build_client, db::connect_db}};

mod handlers;
mod scrapers;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenvy::dotenv().ok();
    let db = connect_db().
        await.expect("Failed to connect to db");
    let db_data = Data::new(db);
    let client = build_client()
        .expect("Failed to build client");
    println!("Client has been built");
    let teams = get_league_teams(&client, "https://www.transfermarkt.com/eredivisie/startseite/wettbewerb/NL1").await.expect("Failed to get the teams");
    println!("Parsed {} teams\n", teams.len());

    let json = serde_json::to_string_pretty(&teams)?;
    println!("{}", json);

    HttpServer::new( move || App::new().app_data(db_data.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
