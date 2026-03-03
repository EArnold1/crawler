use scraper::{Html, Selector};
use url::Url;

use crate::error::CrawlerError;

pub fn parse_html(document: &str) -> Vec<String> {
    let fragment = Html::parse_fragment(document);
    let selector = Selector::parse("a").unwrap();

    let mut urls = Vec::new();

    for element in fragment.select(&selector) {
        if let Some(url) = element.attr("href") {
            if url.contains("mailto:") || url.contains("tel") {
                continue;
            }

            urls.push(url.to_owned());
        }
    }

    urls
}

pub fn extract_host(url: &str) -> Option<String> {
    // TODO: handle url parsing error
    Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(|host| host.to_string()))
}

pub fn hash(host: &String) -> usize {
    let data = blake3::hash(host.as_bytes());

    data.as_bytes()
        .iter()
        .fold(0, |acc, b| acc.wrapping_add(*b as usize))
}

pub fn url_normalizer(origin: &str, url: &str) -> Result<String, CrawlerError> {
    // Resolve URL (absolute or relative)
    let mut parsed_url = match Url::parse(url) {
        Ok(u) => u,
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let base = Url::parse(origin)?;
            base.join(url)?
        }
        Err(e) => return Err(CrawlerError::ParsingUrlError(e)),
    };

    // Lowercase scheme and host
    let _ = parsed_url.set_scheme(&parsed_url.scheme().to_ascii_lowercase());
    if let Some(host) = parsed_url.host_str() {
        let _ = parsed_url.set_host(Some(&host.to_ascii_lowercase()));
    }

    // Remove default ports
    let default_port = matches!(
        (parsed_url.scheme(), parsed_url.port()),
        ("http", Some(80)) | ("https", Some(443))
    );
    if default_port {
        parsed_url.set_port(None).ok();
    }

    // Remove fragment
    parsed_url.set_fragment(None);

    // TODO: Remove query params like "?utm_source"

    // Normalize path (remove trailing slashes except root)
    if let Some(path) = Some(parsed_url.path().to_string())
        && path.len() > 1
    {
        let trimmed = path.trim_end_matches('/');
        parsed_url.set_path(if trimmed.is_empty() { "/" } else { trimmed });
    }

    // Remove trailing slash on root URL
    let mut normalized = parsed_url.to_string();
    if normalized.ends_with('/') && parsed_url.path() == "/" {
        normalized.pop();
    }

    Ok(normalized)
}

#[cfg(test)]
mod url_normalizer_tests {
    use super::*;
    const ORIGIN: &str = "https://example.com";

    #[test]
    fn should_remove_trailing_slash() {
        let dirty_url = format!("{ORIGIN}/path/");

        let normalized_url = url_normalizer(ORIGIN, &dirty_url).expect("Should not fail");

        assert_eq!(format!("{ORIGIN}/path"), normalized_url)
    }

    #[test]
    fn should_remove_fragment() {
        let dirty_url = "#comments";

        let normalized_url = url_normalizer(ORIGIN, dirty_url).expect("Should not fail");

        assert_eq!(ORIGIN, normalized_url)
    }

    #[test]
    fn should_resolve_absolute_path() {
        let dirty_url = "/about";

        let normalized_url = url_normalizer(ORIGIN, dirty_url).expect("Should not fail");

        assert_eq!(format!("{ORIGIN}/about"), normalized_url)
    }

    // should remove default port
    #[test]
    fn should_remove_default_port() {
        let url_with_port = "https://example.com:443";
        let dirty_url = "/port";

        let normalized_url = url_normalizer(url_with_port, dirty_url).expect("Should not fail");

        assert_eq!(format!("{ORIGIN}/port"), normalized_url)
    }
}
