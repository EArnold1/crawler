use std::sync::Arc;

use tokio::sync::mpsc::{self, Sender};

use crate::{
    parser::{extract_host, hash},
    services::worker::spawn_worker,
};

const NUM_WORKERS: usize = 5;

pub struct Queue {
    workers: Arc<Vec<Sender<CrawlTask>>>,
}

impl Clone for Queue {
    fn clone(&self) -> Self {
        Self {
            workers: Arc::clone(&self.workers),
        }
    }
}

// `depth` for a host: It is used to track how deep we are in the crawl for a particular host
// `max_depth`: It is used to limit how deep a crawl should be
#[derive(Clone, Debug)]
pub struct CrawlTask {
    pub url: String,
    pub depth: u8,
}

impl Queue {
    pub fn new(max_depth: u8) -> Self {
        let mut senders = Vec::with_capacity(NUM_WORKERS);
        let mut receivers = Vec::with_capacity(NUM_WORKERS);

        for _ in 0..NUM_WORKERS {
            let (tx, rx) = mpsc::channel(100);
            senders.push(tx);
            receivers.push(rx);
        }

        let queue = Self {
            workers: Arc::new(senders),
        };

        for (id, rx) in receivers.into_iter().enumerate() {
            spawn_worker(id, rx, queue.clone(), max_depth);
        }

        queue
    }

    pub async fn enqueue(&self, task: CrawlTask) {
        if let Some(host) = extract_host(&task.url) {
            let idx = hash(&host) % NUM_WORKERS;
            if let Err(e) = self.workers[idx].send(task).await {
                eprintln!("Failed to send task to worker: {}", e);
            }
        }
    }
}
