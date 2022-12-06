use crate::data::handlers::*;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

static API_KEY: &str = "$2a$10$jR8T/MwHd2DD4ziEB0SqTObIPkn2TYX6rPY/t5YSkznUL7lv9rFQ.";
static URL: &str = "https://api.curseforge.com/v1/mods/search?gameId=432&classId=5&searchFilter=";

pub struct BukkitHandler {}
    
#[derive(Deserialize)]
struct SearchResult {
    data: Vec<Value>
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
            Ok(request) => {request}
            Err(request) => {
                println!("API REQUEST: {}", request);
                std::process::exit(0);
            }
        };
        let body = match request.text().await {
            Ok(body) => {body}
            Err(body) => {
                println!("FETCH BODY: {}", body);
                std::process::exit(0);
            }
        };
        let data: SearchResult = match serde_json::from_str(body.as_str()) {
            Ok(data) => {data}
            Err(data) => {
                println!("PARSE JSON: {}", data);
                std::process::exit(0);
            }
        };


        return data.data;
    }  
}
