use crate::data::{
    handle_request,
    structure::{File, Resource},
};

use super::{AuthorProvider, FileProvider, ResourceProvider, SearchProivder, VersionProvider};
use async_trait::async_trait;
use serde_json::Value;

static URL: &str = "https://api.spiget.org";

pub struct SpigotHandler;

#[async_trait]
impl SearchProivder for SpigotHandler {
    async fn search(&self, filter: &str) -> Vec<Value> {
        let client = reqwest::Client::new();
        let request = client
            .get(URL.to_owned() + "/v2/search/resources/" + filter)
            .send();
        let body = handle_request(request).await;

        let data: Vec<Value> = serde_json::from_str(body.as_str()).expect("Invalid Json Parse");
        return data;
    }
}

#[async_trait]
impl AuthorProvider for SpigotHandler {
    async fn get_author(&self, data: &Value) -> String {
        let author_id = data.get("author").unwrap().get("id").unwrap().to_string();
        let client = reqwest::Client::new();
        let request = client
            .get(URL.to_owned() + "/v2/authors/" + &author_id)
            .send();
        let result = handle_request(request).await;
        return serde_json::from_str::<Value>(result.as_str())
            .expect("Parse Json Error")
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
    }
}

#[async_trait]
impl VersionProvider for SpigotHandler {
    async fn get_version(&self, data: &Value) -> String {
        let id = data.get("id").unwrap().to_string();
        let client = reqwest::Client::new();
        let request = client
            .get(URL.to_owned() + "/v2/resources/" + &id + "/versions/")
            .send();
        let result = handle_request(request).await;
        return serde_json::from_str::<Vec<Value>>(result.as_str())
            .expect("Parse Json Error")
            .get(0)
            .unwrap()
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
    }
}
#[async_trait]
impl FileProvider for SpigotHandler {
    async fn get_file(&self, data: &Value) -> File {
        let file_data = data.get("file").unwrap();
        let file = File {
            size: file_data.get("size").unwrap().as_f64().unwrap(),
            unit: file_data
                .get("sizeUnit")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            url: "https://www.spigotmc.org/".to_string()
                + file_data.get("url").unwrap().as_str().unwrap(),
        };

        return file;
    }
}

#[async_trait]
impl ResourceProvider for SpigotHandler {
    async fn format_data(&self, data: &Value) -> Resource {
        return Resource {
            provider: "spigot".to_string(),
            name: data.get("name").unwrap().as_str().unwrap().to_string(),
            slug: data.get("id").unwrap().to_string(),
            author: self.get_author(data).await,
            file: self.get_file(data).await,
            version: self.get_version(data).await,
        };
    }
}
