use reqwest::Client;
use scraper::Selector;
use crate::{models::competition::{Competition, CompetitionTypes}, scrapers::helpers::table_parser::parse_table_default};

pub async fn get_competition(client: &Client, europe_url: &str) -> Result<Vec<Competition>, reqwest::Error> {
    let mut current_type = CompetitionTypes::FirstTier; 
    
    let competitions = parse_table_default(client, europe_url, |row_map| {
        if let Some(cell) = row_map
            .get("Competition")
            .filter(|c| c.value().attr("colspan") == Some("8"))
            {
                let label = cell.text().collect::<String>().trim().to_string();
                current_type = label.parse().expect("Unknown type");
                return None;
            }
        let cell = &row_map["Competition"];
        let link = cell
            .select(&Selector::parse("a").unwrap())
            .filter(|a| {
                !a 
                    .text()
                    .collect::<String>()
                    .trim()
                    .is_empty()
            })
            .next()
            .expect("missing <a> in league row");
        
        let name = link.text().collect::<String>().trim().to_string();
        // 4. Extract href and split into segments
        let href = link.value().attr("href").unwrap();
        let href = href.trim_start_matches('/'); 
        let segments: Vec<&str> = href.split('/').collect();
        let slug  = segments[0].to_string();
        let tm_id = segments.last().unwrap().to_string();

        let country = row_map["Country"]
            .select(&Selector::parse("img").unwrap())
            .next().unwrap()
            .value().attr("alt").unwrap()
            .to_string();
    
        
        Some(Competition {name, slug, tm_id, country, competition_type: current_type})
    }).await?;
    Ok(competitions)
}
