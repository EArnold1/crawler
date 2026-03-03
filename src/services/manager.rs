use std::time::Duration;

use crate::services::queue::{CrawlTask, Queue};

// TODO: Implement a proper builder pattern for Manager to allow more flexible configuration

pub struct Manager {
    queue: Queue,
}

impl Manager {
    pub fn new(max_depth: u8) -> Self {
        Self {
            queue: Queue::new(max_depth),
        }
    }

    pub async fn run(&self, seeds: Vec<String>) {
        for url in seeds {
            self.queue.enqueue(CrawlTask { url, depth: 0 }).await;
        }

        // Keep alive
        // Come up with a better way for this
        loop {
            std::thread::sleep(Duration::from_secs(60));
        }
    }
}
