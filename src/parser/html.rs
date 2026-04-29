use scraper::{Html, Selector};

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
