use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc::{self, Sender};

use crate::{
    deduplicator::{ContentDeduplicator, InMemoryDeduplicator},
    parser::url::extract_host,
    services::worker::spawn_worker,
    utils::hasher,
};

pub struct Queue {
    workers: Arc<Vec<Sender<(String, u8)>>>,
    visited: Arc<Mutex<HashSet<String>>>, // Track visited URLs
    worker_count: usize,
    deduplicator: Arc<dyn ContentDeduplicator>, // Content deduplicator
}

impl Clone for Queue {
    fn clone(&self) -> Self {
        Self {
            workers: Arc::clone(&self.workers),
            visited: Arc::clone(&self.visited),
            worker_count: self.worker_count,
            deduplicator: Arc::clone(&self.deduplicator),
        }
    }
}

// `depth` for a host: It is used to track how deep we are in the crawl for a particular host
// `max_depth`: It is used to limit how deep a crawl should be

impl Queue {
    pub fn new(max_depth: u8, worker_count: usize) -> Self {
        let mut senders = Vec::with_capacity(worker_count);
        let mut receivers = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            let (tx, rx) = mpsc::channel(500);
            senders.push(tx);
            receivers.push(rx);
        }

        let queue = Self {
            workers: Arc::new(senders),
            visited: Arc::new(Mutex::new(HashSet::new())),
            worker_count,
            deduplicator: Arc::new(InMemoryDeduplicator::default()),
        };

        for (id, rx) in receivers.into_iter().enumerate() {
            spawn_worker(id, rx, queue.clone(), max_depth);
        }

        queue
    }

    pub async fn enqueue(&self, url: String, depth: u8) {
        if let Some(host) = extract_host(&url) {
            let idx = hasher::division_hash(&host, self.worker_count);
            if let Err(e) = self.workers[idx].send((url, depth)).await {
                eprintln!("Failed to send task to worker: {}", e);
            }
        }
    }

    pub fn mark_visited(&self, url: String) {
        let mut visited = self.visited.lock().unwrap();
        visited.insert(url);
    }

    pub fn is_visited(&self, url: &String) -> bool {
        let visited = self.visited.lock().unwrap();
        visited.contains(url)
    }

    /// Mark content as seen using the deduplicator
    pub fn mark_content(&self, content_hash: Vec<u8>) {
        self.deduplicator.mark_seen(content_hash);
    }

    /// Check if content has been seen using the deduplicator
    pub fn is_content_seen(&self, content_hash: &[u8]) -> bool {
        self.deduplicator.is_seen(content_hash)
    }
}
