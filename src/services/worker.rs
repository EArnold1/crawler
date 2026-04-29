use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const POLITENESS_DELAY: Duration = Duration::from_secs(5);

use tokio::sync::mpsc::Receiver;
use url::Url;

use crate::{
    fetcher::{client::fetch_url, page::Page},
    parser::{
        html::parse_html,
        url::{extract_host, url_normalizer},
    },
    services::queue::Queue,
};

// TODO: Implement worker with proper shutdown

pub fn spawn_worker(id: usize, mut rx: Receiver<(String, u8)>, queue: Queue, max_depth: u8) {
    tokio::spawn(async move {
        let mut last_access: HashMap<String, Instant> = HashMap::new();

        while let Some((url, depth)) = rx.recv().await {
            // Using per-url depth tracking to enforce max_depth
            if depth > max_depth {
                println!("[Info]: Skipped {} due to max depth limit", url);
                continue; // Skip URLs beyond max depth
            }
            let origin = Url::parse(&url)
                .expect("URL should be valid")
                .origin()
                .ascii_serialization();

            if let Some(host) = extract_host(&url) {
                enforce_politeness(&mut last_access, &host).await;

                if let Ok(Page { content, .. }) = fetch_url(&url).await {
                    println!("[Worker {id}] Visited {}", url);

                    let content_hash = blake3::hash(content.as_bytes()).as_bytes().to_vec();

                    if queue.is_content_seen(&content_hash) {
                        continue;
                    }

                    queue.mark_content(content_hash);

                    queue.mark_visited(url);

                    for new_url in parse_html(&content) {
                        if let Ok(normalized_url) = url_normalizer(&origin, &new_url)
                            && !queue.is_visited(&normalized_url)
                        {
                            queue.enqueue(normalized_url, depth + 1).await;
                        }
                    }
                }

                last_access.insert(host, Instant::now());
            }
        }
    });
}

async fn enforce_politeness(last_access: &mut HashMap<String, Instant>, host: &str) {
    if let Some(last_time) = last_access.get(host) {
        let elapsed = last_time.elapsed();
        if elapsed < POLITENESS_DELAY {
            tokio::time::sleep(POLITENESS_DELAY - elapsed).await;
        }
    }
}
