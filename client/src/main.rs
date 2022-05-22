mod client;

use std::error::Error;
use client::client::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "http://127.0.0.1:7878".to_string();
    let client =HttpClient::new(&addr);
    for _ in 0..100 {
        client.test_connection().await?;
    }
    //client.add_order(16, "Spaghetti").await?;
    Ok(())
}
