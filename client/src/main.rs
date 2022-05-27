mod client;

use client::client::HttpClient;
use rand::{rngs::StdRng, SeedableRng, Rng};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    test_sequence().await
}

async fn test_sequence() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let scheme = "http";
    let authority = "127.0.0.1:7878";
    let client =HttpClient::new(scheme, authority);
    client.test_connection().await?;

    // Testing sequence
    for _ in 1..15  {
        let mut rng = StdRng::from_entropy();

        // Has a 1 in 10000000000 chance to fail due to duplicate table_id.
        let table_id = rng.gen_range(1..100000);

        println!("{}", client.add_order(table_id, "Katsudon").await?);
        println!("{}", client.get_remaining_table_orders(table_id).await?);
        println!("{}", client.get_items_for_table(table_id, "Katsudon").await?);
        let orders = client.get_orders().await?;
        println!("{}", orders);
        let orders = orders.lines().find(|s| s.contains(format!("1:{}", table_id).as_str())).unwrap().to_string();
        let orders = orders.chars().skip(2).next().unwrap().to_string();
        let orders: i32 = orders.parse().unwrap();
        println!("{}", client.delete_order(orders).await?);
    }
    
    Ok(())
}