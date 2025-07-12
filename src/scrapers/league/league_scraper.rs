use reqwest::Client;
use scraper::Selector;

use crate::{models::league::League, scrapers::helpers::table_parser::parse_table_default};

pub async fn get_league(client: &Client, europe_url: &str) -> Result<Vec<League>, reqwest::Error> {
    let leagues = parse_table_default(client, europe_url, |row_map| {
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
    }).await?;
    Ok(leagues)
}
