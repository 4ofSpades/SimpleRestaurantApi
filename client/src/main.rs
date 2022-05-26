mod client;

use client::client::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let scheme = "http";
    let authority = "127.0.0.1:7878";
    let client =HttpClient::new(scheme, authority);
    client.test_connection().await?;

    // Testing sequence
    println!("{}", client.add_order(1, "Katsudon").await?);
    println!("{}", client.get_remaining_table_orders(1).await?);
    println!("{}", client.get_items_for_table(1, "Spaghetti").await?);
    println!("{}", client.delete_order(1).await?);
    Ok(())
}
