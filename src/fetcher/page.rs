//! Representation of a fetched web page.

use reqwest::header::HeaderMap;

#[derive(Debug, Clone)]
pub struct Page {
    pub url: String,
    pub status_code: u16,
    pub headers: HeaderMap,
    pub content: String,
}

impl Page {
    /// Returns true if the content type is HTML.
    pub fn is_html(&self) -> bool {
        self.headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|ct| ct.contains("text/html"))
            .unwrap_or(false)
    }
}
