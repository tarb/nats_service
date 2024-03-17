use std::convert::Infallible;
use tokio::sync::RwLock;

use super::Repository;

pub struct Memory {
    count: RwLock<i32>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            count: RwLock::new(0),
        }
    }
}

impl Repository for Memory {
    type Error = Infallible;

    async fn count(&self) -> Result<i32, Self::Error> {
        let count = self.count.read().await;

        Ok(*count)
    }

    async fn increment(&self, amount: i32) -> Result<i32, Self::Error> {
        let mut count = self.count.write().await;
        let count = {
            *count += amount;
            *count
        };
        Ok(count)
    }

    async fn decrement(&self, amount: i32) -> Result<i32, Self::Error> {
        let mut count = self.count.write().await;
        let count = {
            *count -= amount;
            *count
        };
        Ok(count)
    }
}
