mod client;

use std::error::Error;
use client::client::TcpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let addr = "127.0.0.1:7878".to_string();
    TcpClient::connect(&addr).await
}
