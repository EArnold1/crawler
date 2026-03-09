use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc::{self, Sender};

use crate::{
    parser::{extract_host, hash},
    services::worker::spawn_worker,
};

const NUM_WORKERS: usize = 5;

pub struct Queue {
    workers: Arc<Vec<Sender<String>>>,
    depth: u8,                            // Track depth
    visited: Arc<Mutex<HashSet<String>>>, // Track visited URLs
}

impl Clone for Queue {
    fn clone(&self) -> Self {
        Self {
            workers: Arc::clone(&self.workers),
            depth: self.depth,
            visited: Arc::clone(&self.visited),
        }
    }
}

// `depth` for a host: It is used to track how deep we are in the crawl for a particular host
// `max_depth`: It is used to limit how deep a crawl should be

impl Queue {
    pub fn new(max_depth: u8) -> Self {
        let mut senders = Vec::with_capacity(NUM_WORKERS);
        let mut receivers = Vec::with_capacity(NUM_WORKERS);

        for _ in 0..NUM_WORKERS {
            let (tx, rx) = mpsc::channel(500);
            senders.push(tx);
            receivers.push(rx);
        }

        let queue = Self {
            workers: Arc::new(senders),
            depth: 0,
            visited: Arc::new(Mutex::new(HashSet::new())),
        };

        for (id, rx) in receivers.into_iter().enumerate() {
            spawn_worker(id, rx, queue.clone(), max_depth);
        }

        queue
    }

    pub async fn enqueue(&self, url: String) {
        if let Some(host) = extract_host(&url) {
            let idx = hash(&host) % NUM_WORKERS;
            if let Err(e) = self.workers[idx].send(url).await {
                eprintln!("Failed to send task to worker: {}", e);
            }
        }
    }

    pub fn increment_depth(&mut self) {
        self.depth += 1;
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }

    pub fn mark_visited(&self, url: String) {
        let mut visited = self.visited.lock().unwrap();
        visited.insert(url);
    }

    pub fn is_visited(&self, url: &String) -> bool {
        let visited = self.visited.lock().unwrap();
        visited.contains(url)
    }
}
