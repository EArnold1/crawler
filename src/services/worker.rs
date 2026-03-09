use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const POLITENESS_DELAY: Duration = Duration::from_secs(5);

use tokio::sync::mpsc::Receiver;
use url::Url;

use crate::{
    fetcher::fetch_page,
    parser::{extract_host, parse_html, url_normalizer},
    services::queue::Queue,
};

// TODO: Implement worker pool with async tasks and proper shutdown mechanism

pub fn spawn_worker(id: usize, mut rx: Receiver<String>, mut queue: Queue, max_depth: u8) {
    tokio::spawn(async move {
        let mut last_access: HashMap<String, Instant> = HashMap::new();

        while let Some(url) = rx.recv().await {
            println!("Depth: {}, id: {}", queue.depth(), id); // TODO: remove
            if queue.depth() >= max_depth {
                // TODO: implement a way to shutdown the worker
                continue;
            }

            let origin = Url::parse(&url)
                .expect("URL should be valid")
                .origin()
                .ascii_serialization();

            if let Some(host) = extract_host(&url) {
                enforce_politeness(&mut last_access, &host).await;

                if let Ok(document) = fetch_page(&url).await {
                    println!("[Worker {id}] Visited {}", url);
                    queue.increment_depth();
                    queue.mark_visited(url);

                    for new_url in parse_html(&document) {
                        // TODO: Implement seen URL + Content hash check to avoid duplicates
                        if let Ok(normalized_url) = url_normalizer(&origin, &new_url)
                            && !queue.is_visited(&normalized_url)
                        {
                            queue.enqueue(normalized_url).await;
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
