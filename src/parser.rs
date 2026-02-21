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

pub fn url_normalizer(origin: &str, url: &str) -> Result<String, CrawlerError> {
    // Try to parse as absolute URL, otherwise resolve relative to host
    let mut parsed_url = match Url::parse(url) {
        Ok(u) => u,
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let base = Url::parse(origin)?;
            base.join(url)?
        }
        Err(e) => return Err(CrawlerError::ParsingUrlError(e)),
    };

    // Lowercase scheme and host
    let scheme = parsed_url.scheme().to_ascii_lowercase();
    parsed_url.set_scheme(&scheme).ok();
    if let Some(host) = parsed_url.host_str() {
        parsed_url.set_host(Some(&host.to_ascii_lowercase())).ok();
    }

    // Remove default ports
    let is_default_port = match parsed_url.scheme() {
        "http" => parsed_url.port() == Some(80),
        "https" => parsed_url.port() == Some(443),
        _ => false,
    };
    if is_default_port {
        parsed_url.set_port(None).ok();
    }

    // Remove fragment
    parsed_url.set_fragment(None);

    // Remove trailing slash (but not for root path)
    if let Some(mut path) = Some(parsed_url.path().to_string())
        && path.ends_with('/')
        && path != "/"
    {
        path.pop();
        while path.ends_with('/') && path != "/" {
            path.pop();
        }
        parsed_url.set_path(&path);
    }

    Ok(parsed_url.to_string())
}
