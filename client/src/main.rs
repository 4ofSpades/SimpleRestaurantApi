mod client;

use std::error::Error;
use client::client::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let addr = "127.0.0.1:7878".to_string();
    let client =HttpClient::new(&addr);
    client.add_order(16, "Spaghetti").await;
    Ok(())
}
