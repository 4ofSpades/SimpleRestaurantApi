mod client;

use client::client::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let scheme = "http";
    let authority = "127.0.0.1:7878";
    let client =HttpClient::new(scheme, authority);
    client.test_connection().await?;
    println!("{}", client.get_items_for_table(999, "Spaghetti").await?);
    //client.add_order(16, "Spaghetti").await?;
    Ok(())
}
