use std::collections::HashSet;

use url::Url;

use crate::{
    error::CrawlerError,
    fetcher::fetch_page,
    parser::{parse_html, url_normalizer},
};

const DEFAULT_MAX_DEPTH_LEVEL: u8 = 10;

pub struct Manager {
    seed: Vec<String>,
    depth: u8,
    max_depth: u8,
    visited: HashSet<String>,
}

impl Manager {
    pub fn builder() -> ManagerBuilder {
        ManagerBuilder::default()
    }
}

pub struct ManagerBuilder {
    seed: Vec<String>,
    max_depth: u8,
}

impl Default for ManagerBuilder {
    fn default() -> Self {
        Self {
            seed: Vec::new(),
            max_depth: DEFAULT_MAX_DEPTH_LEVEL,
        }
    }
}

impl ManagerBuilder {
    pub fn new(seeded_urls: Vec<String>) -> ManagerBuilder {
        ManagerBuilder {
            seed: seeded_urls,
            max_depth: DEFAULT_MAX_DEPTH_LEVEL,
        }
    }

    pub fn set_max_depth(mut self, max_depth: u8) -> ManagerBuilder {
        assert!(max_depth > 0);
        self.max_depth = max_depth;
        self
    }

    pub fn seed_url(mut self, url: &str) -> ManagerBuilder {
        self.seed.push(url.into());
        self
    }

    pub fn build(self) -> Manager {
        Manager {
            seed: self.seed,
            max_depth: self.max_depth,
            visited: HashSet::new(),
            depth: 0,
        }
    }
}

impl Manager {
    pub async fn run(&mut self) -> Result<(), CrawlerError> {
        while let Some(url) = self.seed.pop() {
            if self.depth >= self.max_depth {
                break;
            }

            let origin = Url::parse(&url)?.origin().ascii_serialization();

            let normalized_url = url_normalizer(&origin, &url)?;

            let document = fetch_page(&normalized_url).await?;
            self.depth += 1;

            println!("[Info]: Visited {}", &url);
            self.visited.insert(url);

            for new_url in parse_html(&document) {
                let normalized_url = url_normalizer(&origin, &new_url)?;

                if !self.visited.contains(&normalized_url) {
                    self.seed.push(normalized_url);
                }
            }
        }

        println!("[Info]: Done...");
        println!("{:#?}", self.visited);
        Ok(())
    }
}

//
/*
 * ISSUES
 * Infinite loop in a particular platform
 */
