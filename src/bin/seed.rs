use dotenvy::dotenv;
use football_app_api::{entity::{competitions, seasons}, scrapers::league::competition_scraper::get_competition, seeders::competition_seeder::seed_competition, utils::{client::build_client, db::connect_db}};
use sea_orm::EntityTrait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db = connect_db().
        await.expect("Failed to connect to db");

    competitions::Entity::delete_many().exec(&db).await?;
    seasons::Entity::delete_many().exec(&db).await?;

    let client = build_client()
        .expect("Failed to build client");
    println!("Client has been built");
    let comps = get_competition(&client, "https://www.transfermarkt.com/wettbewerbe/europa").await.expect("Failed to get the leagues");
    println!("Scraped {} competitions", comps.len());

    seed_competition(&db, comps)
        .await
        .expect("Failed to seed the competitions");
    println!("Seeded competitions into the database");
    Ok(())

}