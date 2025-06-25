use reqwest::Client;

//Uses our client builder so when we scrape we always give the same headers and options we made in our builder every time we use this function.
pub async fn fetch_html(client: &Client, url: &str) -> Result<String, reqwest::Error> {
    let html = 
        client.get(url).send().await?.text().await?;

    Ok(html)
} 