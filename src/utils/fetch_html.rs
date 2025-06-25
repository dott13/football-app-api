use reqwest::Client;


pub async fn fetch_html(client: &Client, url: &str) -> Result<String, reqwest::Error> {
    let html = 
        client.get(url).send().await?.text().await?;

    Ok(html)
} 