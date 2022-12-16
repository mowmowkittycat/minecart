use crate::data::handlers::*;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

static API_KEY: &str = "$2a$10$jR8T/MwHd2DD4ziEB0SqTObIPkn2TYX6rPY/t5YSkznUL7lv9rFQ.";
static URL: &str = "https://api.curseforge.com/v1/mods/search?gameId=432&classId=5&searchFilter=";

pub struct BukkitHandler;

#[derive(Deserialize)]
struct SearchResult {
    data: Vec<Value>,
}

#[async_trait]
impl SearchProivder for BukkitHandler {
    async fn search(&self, filter: &str) -> Vec<Value> {
        let client = reqwest::Client::new();
        let request = match client
            .get(URL.to_owned() + filter)
            .header("x-api-key", API_KEY)
            .send()
            .await
        {
            Ok(request) => request,
            Err(request) => {
                println!("API REQUEST: {}", request);
                std::process::exit(0);
            }
        };
        let body = match request.text().await {
            Ok(body) => body,
            Err(body) => {
                println!("FETCH BODY: {}", body);
                std::process::exit(0);
            }
        };
        let data: SearchResult = serde_json::from_str(body.as_str()).expect("Parse Json Error");

        return data.data;
    }
}

#[async_trait]
impl AuthorProvider for BukkitHandler {
    async fn get_author(&self, data: &Value) -> String {
        return data
            .get("authors")
            .unwrap()
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
impl FileProvider for BukkitHandler {
    async fn get_file(&self, data: &Value) -> File {
        let latest_files = data.get("latestFiles").unwrap();
        let file = File {
            size: latest_files
                .get(0)
                .unwrap()
                .get("fileLength")
                .unwrap()
                .as_i64()
                .unwrap() as f64,
            unit: "b".to_string(),
            url: "bob".to_string(),
        };

        return file;
    }
}

#[async_trait]
impl VersionProvider for BukkitHandler {
    async fn get_version(&self, data: &Value) -> String {
        let latest_files = data.get("latestFiles").unwrap();
        return latest_files
            .get(0)
            .unwrap()
            .get("id")
            .unwrap()
            .to_string();
    }
}

#[async_trait]
impl ResourceProvider for BukkitHandler {
    async fn format_data(&self, data: &Value) -> Resource {
        return Resource {
            provider: "bukkit".to_string(),
            name: data.get("name").unwrap().as_str().unwrap().to_string(),
            slug: data.get("id").unwrap().to_string(),
            author: self.get_author(data).await,
            file: self.get_file(data).await,
            version: self.get_version(data).await,
        };
    }
}
