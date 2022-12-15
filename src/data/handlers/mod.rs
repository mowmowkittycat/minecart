use serde_json::{Value};
use async_trait::async_trait;
use super::structure::{Resource, File};

pub mod bukkit;
pub mod spigot;


pub trait Provider: SearchProivder + ResourceProvider {}
impl<T> Provider for T where T: SearchProivder + ResourceProvider {}


#[async_trait]
pub trait SearchProivder {
    async fn search(&self, filter: &str) -> Vec<Value>;
}

#[async_trait]
pub trait ResourceProvider {
    async fn format_data(&self, data: &Value) -> Resource;
}

#[async_trait]
pub trait AuthorProvider {
    async fn getAuthor(&self, data: &Value) -> String;

}

#[async_trait]
pub trait VersionProvider {
    async fn getVersion(&self, data: &Value) -> String; 
}

#[async_trait]
pub trait FileProvider {
    async fn getFile(&self, data: &Value) -> File;
}
