use scraper::{Html, Selector};

pub fn find_next_page(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let sel = Selector::parse("li.tm-pagination__list-item.tm-pagination__list-item--icon-next-page a.tm-pagination__link").expect("Next page not found");
    document
        .select(&sel)
        .next()
        .and_then(|a| a.value().attr("href"))
        .map(|href| format!( "{}{}", std::env::var("DEFAULT_URL").expect("DEFAULT_URL variable should be set in .env"), href))
}