use std::collections::HashMap;

use scraper::{ElementRef, Html, Selector};

pub fn parse_table<T, F>(
    html: &str,
    table_selector: &str,
    header_selector: &str,
    row_selector: &str,
    map_row: F
) -> Vec<T>
where 
    F: Fn(&HashMap<String, scraper::ElementRef>) -> T 
{
    let document = Html::parse_document(html);

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
        println!("headers: {}", label);
        for _ in 0..span {
            headers.push(label.clone());
        }
    }

    let cells_sel = Selector::parse(":scope > td").unwrap();
    let mut results: Vec<T> = Vec::new();
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

    results
}

pub fn parse_table_default<T, F>(
    html: &str,
    map_row: F
) -> Vec<T>
where F: Fn(&HashMap<String, ElementRef>) -> T {
    parse_table(
        html,
        "table.items",
        "thead tr th",
        "tbody > tr",
        map_row
    )
}