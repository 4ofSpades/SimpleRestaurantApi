mod client;

use client::client::HttpClient;

#[tokio::test]
async fn test_single_threaded_restaurant() {
    let scheme = "http";
    let authority = "127.0.0.1:7878";
    let client = HttpClient::new(scheme, authority);

    // Connect
    client.test_connection().await?;
    // Add
    client.add_order()
    // Get remaining 
    // Sleep for max duration
    // Get remaining
    // Get for table
    // Delete 
    // Get for table
    assert!(true);
}