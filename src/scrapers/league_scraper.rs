use reqwest::Client;
use scraper::{Html, Selector};

use crate::{models::team::Team, utils::{fetch_html::fetch_html, parse_money::parse_money}};

pub async fn get_league_teams(client: &Client, league_url: &str) -> Result<Vec<Team>, reqwest::Error> {
    let html = fetch_html(client, league_url).await?;
    let teams = parse_teams(&html);
    Ok(teams)
}

fn parse_teams(html: &str) -> Vec<Team> { 
    //Get html document we want to scrape from
    let document = Html::parse_document(html);

    //Select only the first table that we see on our page
    let table_sel = Selector::parse("table.items").unwrap();
    let league_table = document
        .select(&table_sel)
        .next()
        .expect("No league table found");

    // We get our headers here to later use them when we group the data that we fetch from the initial table
    let header = Selector::parse("table.items thead tr th").unwrap();
    let mut headers = Vec::new();
    //We get the labels here so we can adjust them when we arrange the data
    for th in league_table.select(&header) {
        let label = th
            .text()
            .collect::<String>()
            .trim()
            .to_string();
        //filter based on the label so we get only the clean ones we need
        if label.eq_ignore_ascii_case("name") || label == "#" || label.is_empty() {
            continue;
        }
        //Using the colspan of the table headers we actually get the true number of labels in the headers we need
        let span = th
            .value()
            .attr("colspan")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);
        //push the labels to the headers vector
        for _ in 0..span {
            headers.push(label.clone());
        }
    }

    //We get the indexes of the table headers we need in here
    //Club is on the board two times because it spans over two columns for the crest and the name so here we filter and collect them in a Vec
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
    //Here we take the row selector for the league table variable we made earlier for the sake of taking only the first table we encounter
    for row in league_table.select(&row_sel) {
        let cells: Vec<_> = row.select(&td_sel).collect();
        //We use here the cells Vec to get all the cells appart for every column selecting the names text collecting them and getting them into the value we need
        let name_cell = &cells[name_id];
        let name = name_cell
            .select(&Selector::parse("a").unwrap())
            .next().unwrap()
            .text().collect::<String>().trim().to_string();
        //For the image we dont need the text or label, rather we need the src attribute of the image so we can get the url needed.
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
        //We use our parse_money() helper to split the value and currency for our team model.
        let delta_value = parse_money(&(cells[delta_value_id]
            .text().collect::<String>()));
        let market_value = parse_money(&(cells[market_value_id]
            .text().collect::<String>()));
        //push the teams we collected and made just now and return them.
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
