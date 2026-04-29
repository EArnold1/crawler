//! HTTP client for fetching web pages.

use crate::error::CrawlerError;
use crate::fetcher::page::Page;
use once_cell::sync::Lazy;
use reqwest::{Client, Response};
use std::sync::Arc;

// Singleton HTTP client for connection reuse
static HTTP_CLIENT: Lazy<Arc<Client>> = Lazy::new(|| {
    Arc::new(
        Client::builder()
            .user_agent("rsCrawler")
            .build()
            .expect("Failed to build HTTP client"),
    )
});

/// Fetches a URL and returns a Page struct on success.
pub async fn fetch_url(url: &str) -> Result<Page, CrawlerError> {
    let resp = HTTP_CLIENT
        .get(url)
        .send()
        .await
        .map_err(CrawlerError::HttpError)?;
    build_page(url, resp).await
}

async fn build_page(url: &str, resp: Response) -> Result<Page, CrawlerError> {
    let status = resp.status();
    let headers = resp.headers().clone();
    let content = resp.text().await.map_err(CrawlerError::HttpError)?;
    Ok(Page {
        url: url.to_string(),
        status_code: status.as_u16(),
        headers,
        content,
    })
}
