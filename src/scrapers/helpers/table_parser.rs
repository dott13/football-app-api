use std::collections::HashMap;

use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use crate::{scrapers::helpers::next_page::find_next_page, utils::fetch_html::fetch_html};


pub async fn parse_table<T, F>(
    client: &Client,
    start_url: &str,
    table_selector: &str,
    header_selector: &str,
    row_selector: &str,
    map_row: F
) -> Result<Vec<T>, reqwest::Error>
where 
    F: Fn(&HashMap<String, scraper::ElementRef>) -> T 
{
    let mut url = start_url.to_string();
    let mut results = Vec::new();

    loop {
        let html = fetch_html(client, &url).await?;

        let document = Html::parse_document(&html);
        let table = document.
            select(&Selector::parse(table_selector).unwrap())
            .next()
            .expect("No table found");  

        let header = Selector::parse(header_selector).expect("No header found");

        let row_sel = Selector::parse(row_selector).expect("Invalid Selector");

        let mut headers = Vec::new();
        for th in table.select(&header) {
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

        let cells_sel = Selector::parse(":scope > th, :scope > td").unwrap();
        for row in table.select(&row_sel) {

            let cells: Vec<ElementRef> = row.select(&cells_sel).collect();

            if cells.len() != headers.len() {
                continue;
            }

            let mut row_map = HashMap::new();
            for (i, hdr) in headers.iter().enumerate() {
                row_map.insert(hdr.clone(), cells[i].clone());
            }

            results.push(map_row(&row_map));
        }
        
        if let Some(next) = find_next_page(&html) {
            url = next;
            continue;
        } else {
            break;
        }
    }

    Ok(results)
}

pub async fn parse_table_default<T, F>(
    client: &Client,
    html: &str,
    map_row: F
) -> Result<Vec<T>, reqwest::Error>
where F: Fn(&HashMap<String, ElementRef>) -> T {
    parse_table(
        client,
        html,
        "table.items",
        "thead tr th",
        ":scope > tbody > tr",
        map_row
    ).await
}