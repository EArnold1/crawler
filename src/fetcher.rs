use crate::error::CrawlerError;

pub async fn fetch_page(url: &str) -> Result<String, CrawlerError> {
    let client = reqwest::Client::builder().build()?; // TODO: manage this so it is created just once and reused

    let response = client.get(url).send().await?.text().await?;

    Ok(response)
}
