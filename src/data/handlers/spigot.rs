use super::SearchProivder;
use serde_json::Value;
use async_trait::async_trait;


static URL: &str = "https://api.spiget.org/v2/search/resources/";



pub struct SpigotHandler {}

#[async_trait]
impl SearchProivder for SpigotHandler {
   async fn search(&self, filter: &str) -> Vec<Value> { 
        let client = reqwest::Client::new();
        let request = match client
            .get(URL.to_owned() + filter)
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
        let data: Vec<Value> = match serde_json::from_str(body.as_str()) {
            Ok(data) => {data}
            Err(data) => {
                println!("PARSE JSON: {}", data);
                std::process::exit(0);
            }
        };

        return data
     
   }
}
