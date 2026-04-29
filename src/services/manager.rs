use std::time::Duration;

use crate::services::queue::Queue;

pub struct Manager {
    queue: Queue,
}

impl Manager {
    pub async fn run<I>(&self, seeds: I)
    where
        I: IntoIterator<Item = String>,
    {
        for url in seeds {
            self.queue.enqueue(url).await;
        }

        // Keep alive
        // TODO: Come up with a better way for this
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }
}

pub struct ManagerBuilder {
    max_depth: u8,
    worker_count: usize, // max size should be 10
}

impl Default for ManagerBuilder {
    fn default() -> Self {
        Self {
            max_depth: 3,
            worker_count: 5,
        }
    }
}

impl ManagerBuilder {
    pub fn new() -> Self {
        ManagerBuilder::default()
    }

    pub fn max_depth(mut self, depth: u8) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn max_workers(mut self, workers: usize) -> Self {
        self.worker_count = workers.min(10);
        self
    }

    pub fn build(self) -> Manager {
        Manager {
            queue: Queue::new(self.max_depth, self.worker_count),
        }
    }
}
