use reqwest::Client;
use scraper::{Html, Selector};

use crate::{models::team::Team, utils::{fetch_html::fetch_html, parse_money::parse_money}};

pub async fn get_league_teams(client: &Client, league_url: &str) -> Result<Vec<Team>, reqwest::Error> {
    let html = fetch_html(client, league_url).await?;
    let teams = parse_teams(&html);
    Ok(teams)
}

fn parse_teams(html: &str) -> Vec<Team> { 
    let document = Html::parse_document(html);

    let table_sel = Selector::parse("table.items").unwrap();
    let league_table = document
        .select(&table_sel)
        .next()
        .expect("No league table found");

    let header = Selector::parse("table.items thead tr th").unwrap();
    let mut headers = Vec::new();
    for th in league_table.select(&header) {
        let label = th
            .text()
            .collect::<String>()
            .trim()
            .to_string();

        if label.eq_ignore_ascii_case("name") || label == "#" || label.is_empty() {
            continue;
        }

        let span = th
            .value()
            .attr("colspan")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);

        for _ in 0..span {
            headers.push(label.clone());
        }
    }

    println!("Parsed headers ({}): {:?}", headers.len(), headers);

    let club_ids: Vec<_> = headers.iter().enumerate().filter(|(_,h)| h.contains("Club")).map(|(i, _)| i).collect();
    let crest_id = club_ids[0];
    let name_id = club_ids[1];
    let squad_size_id = headers.iter().position(|h| h.contains("Squad")).unwrap();
    let delta_age_id = headers.iter().position(|h| h.contains("ø age")).unwrap();
    let foreigners_id = headers.iter().position(|h| h.contains("Foreigners")).unwrap();
    let delta_value_id = headers.iter().position(|h| h.contains("ø market value")).unwrap();
    let market_value_id = headers.iter().position(|h| h.contains("Total market value") ).unwrap();
    
    let row_sel = Selector::parse("table.items > tbody > tr").unwrap();
    let td_sel = Selector::parse("td").unwrap();

    let mut teams:Vec<Team> = Vec::new();
    for row in league_table.select(&row_sel) {
        println!("ROW:\n{}\n", row.html());
        let cells: Vec<_> = row.select(&td_sel).collect();
        let name_cell = &cells[name_id];
        let name = name_cell
            .select(&Selector::parse("a").unwrap())
            .next().unwrap()
            .text().collect::<String>().trim().to_string();
        let logo_cell = &cells[crest_id];
        let logo = logo_cell
            .select(&Selector::parse("img").unwrap())
            .next().and_then(|img| img.value().attr("src")).unwrap_or_default();

        let squad_size = cells[squad_size_id]
            .text().collect::<String>().trim().parse::<u8>().ok();

        let delta_age = cells[delta_age_id]
            .text().collect::<String>().trim().parse::<f32>().ok();

        let foreigners = cells[foreigners_id]
            .text().collect::<String>().trim().parse::<u8>().ok();

        let delta_value = parse_money(&(cells[delta_value_id]
            .text().collect::<String>()));
        let market_value = parse_money(&(cells[market_value_id]
            .text().collect::<String>()));

        teams.push(Team {
            name,
            logo: logo.to_string(),
            squad_size,
            delta_age,
            foreigners,
            delta_value,
            market_value
        });
    }

    teams
}
