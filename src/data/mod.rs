use reqwest;
use serde_json::{Value};

pub mod handlers;
use crate::data::handlers::SearchProivder;



pub async fn search(filter: &str) -> Vec<Vec<Value>> {
   let providers: Vec<Box<dyn SearchProivder>> = vec![
        Box::new(handlers::bukkit::BukkitHandler {}),
        Box::new(handlers::spigot::SpigotHandler {})

   
   ];

   let mut data: Vec<Vec<Value>> = vec![];

   for provider in providers.iter() {
       let provider_data = provider.search(filter).await;
       data.push(provider_data);
   }

   return data;



}
