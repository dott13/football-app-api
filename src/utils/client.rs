use reqwest::{header::{self, HeaderValue}, Error};

pub fn build_client() -> Result<reqwest::Client, Error> {

    let mut headers = header::HeaderMap::new();
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.5"));

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:139.0) Gecko/20100101 Firefox/139.0")
        .default_headers(headers)
        .build()?;

    Ok(client)
}