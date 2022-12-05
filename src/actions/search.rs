use reqwest;
use serde_json::{Value};

pub async fn exec(args: Vec<String>) {
    let url = "https://api.spiget.org/v2/search/resources/".to_string() + &args.join(" ");
    let request = match reqwest::get(url).await {
        Ok(f) => {f}
        Err(f) => {
            println!("{}", f);
            std::process::exit(0);
        }
    };
    let body = request.text().await.expect("Malformed Body");
    let data: Vec<Value>  = serde_json::from_str(body.as_str()).expect("Malformed JSON");
    for resource in data {
        println!("{}", resource["name"]);
    }


    
}

