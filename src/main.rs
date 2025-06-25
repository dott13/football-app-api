use actix_web::{App, HttpServer};

use crate::{scrapers::league_scraper::get_league_teams, utils::{client::build_client, fetch_html::fetch_html}};

mod handlers;
mod scrapers;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    
    let client = build_client()
        .expect("Failed to build client");
    println!("Client has been built");
    let html = fetch_html(&client, "https://www.transfermarkt.com/eredivisie/startseite/wettbewerb/NL1").await;
    println!("fetched html");

    let teams = get_league_teams(&client, "https://www.transfermarkt.com/eredivisie/startseite/wettbewerb/NL1").await.expect("Failed to get the teams");
    println!("Parsed {} teams\n", teams.len());

    for (i, team) in teams.iter().enumerate() {
        println!("Team #{}:", i + 1);
        println!("  Name:         {}", team.name);
        println!("  Logo URL:     {}", team.logo);
        println!("  Squad size:   {:?}", team.squad_size);
        println!("  Δ Age:        {:?}", team.delta_age);
        println!("  Foreigners:   {:?}", team.foreigners);
        println!("  Δ Value:      {:?}", team.delta_value);
        println!("  Market value: {:?}\n", team.market_value);
    }
    
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
