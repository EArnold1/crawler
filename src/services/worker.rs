use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

const POLITENESS_DELAY: Duration = Duration::from_secs(2);

use tokio::sync::mpsc::Receiver;

use crate::{
    fetcher::fetch_page,
    parser::{extract_host, parse_html, url_normalizer},
    services::queue::{CrawlTask, Queue},
};

// TODO: Implement worker pool with async tasks and proper shutdown mechanism

pub fn spawn_worker(id: usize, mut rx: Receiver<CrawlTask>, queue: Queue, max_depth: u8) {
    tokio::spawn(async move {
        let mut last_access: HashMap<String, Instant> = HashMap::new();

        while let Some(task) = rx.recv().await {
            if task.depth >= max_depth {
                continue;
            }

            if let Some(host) = extract_host(&task.url) {
                enforce_politeness(&mut last_access, &host);

                if let Ok(document) = fetch_page(&task.url).await {
                    println!("[Worker {id}] Visited {}", task.url);

                    for new_url in parse_html(&document) {
                        // TODO: Implement seen URL + Content hash check to avoid duplicates
                        if let Ok(normalized_url) = url_normalizer(&task.url, &new_url) {
                            queue
                                .enqueue(CrawlTask {
                                    url: normalized_url,
                                    depth: task.depth + 1,
                                })
                                .await;
                        }
                    }
                }

                last_access.insert(host, Instant::now());
            }
        }
    });
}

fn enforce_politeness(last_access: &mut HashMap<String, Instant>, host: &str) {
    if let Some(last_time) = last_access.get(host) {
        let elapsed = last_time.elapsed();
        if elapsed < POLITENESS_DELAY {
            thread::sleep(POLITENESS_DELAY - elapsed);
        }
    }
}
