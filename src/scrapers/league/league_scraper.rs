use reqwest::Client;
use scraper::Selector;

use crate::{models::league::League, scrapers::helpers::table_parser::parse_table_default, utils::fetch_html::fetch_html};

pub async fn get_league(client: &Client, europe_url: &str) -> Result<Vec<League>, reqwest::Error> {
    let html = fetch_html(client, europe_url).await?;
    let leagues = parse_league(&html);
    Ok(leagues)
}

fn parse_league(html: &str) -> Vec<League> {
    let leagues: Vec<League> = parse_table_default(&html, |row_map| {
        let cell = &row_map["Competition"];

        let link = cell
            .select(&Selector::parse("a").unwrap())
            .next()
            .expect("missing <a> in league row");
        
        let name = link.text().collect::<String>().trim().to_string();

        // 4. Extract href and split into segments
        let href = link.value().attr("href").unwrap();
        let href = href.trim_start_matches('/'); 
        let segments: Vec<&str> = href.split('/').collect();
        let slug  = segments[0].to_string();
        let tm_id = segments.last().unwrap().to_string();
    
        
        League {name, slug, tm_id}
    });

    leagues
}