use std::future::Future;

use reqwest::{Error, Response};

use self::structure::Resource;
use crate::data::handlers::*;
use serde_json::Value;

pub mod handlers;
pub mod structure;

pub async fn handle_request(request: impl Future<Output = Result<Response, Error>>) -> String {
    let result = match request.await {
        Ok(result) => result,
        Err(result) => {
            println!("API REQUEST: {}", result);
            std::process::exit(0);
        }
    };
    let body = match result.text().await {
        Ok(body) => body,
        Err(body) => {
            println!("FETCH BODY: {}", body);
            std::process::exit(0);
        }
    };

    body
}

struct ProviderData {
    provider: Box<dyn Provider>,
    list: Vec<Value>,
}

pub async fn search(filter: &str) -> Vec<Vec<Resource>> {
    let providers: Vec<Box<dyn Provider>> = vec![
        Box::new(bukkit::BukkitHandler),
        Box::new(spigot::SpigotHandler),
    ];
    let size: usize = 10;
    let mut data_size: usize = 0;
    let mut data: Vec<ProviderData> = vec![];
    let mut formatted_data: Vec<Vec<Resource>> = vec![];

    for provider in providers {
        let provider_data = provider.search(filter).await;
        data.push(ProviderData {
            provider,
            list: provider_data,
        });
    }

    for data in data.iter() {
        data_size += data.list.len();
    }

    if data_size <= size {
        for data in data.iter() {
            let mut list: Vec<Resource> = vec![];
            for item in data.list.iter() {
                list.push(data.provider.format_data(item).await);
            }
            formatted_data.push(list);
        }
        return formatted_data;
    }

    let per_data_size: usize = size / data.len();

    for data in data.iter() {
        let mut list: Vec<Resource> = vec![];
        for (loop_num, item) in data.list.iter().enumerate() {
            if loop_num > per_data_size {
                break;
            }

            list.push(data.provider.format_data(item).await);
        }
        formatted_data.push(list);
    }

    formatted_data
}
