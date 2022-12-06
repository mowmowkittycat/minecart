use serde_json::{Value};
use async_trait::async_trait;

pub mod bukkit;
pub mod spigot;

#[async_trait]
pub trait SearchProivder {
    async fn search(&self, filter: &str) -> Vec<Value>;
}
