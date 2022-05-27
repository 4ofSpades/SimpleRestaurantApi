pub mod client {
    use hyper::{Client, Request, Method, Body, Uri, body, http::uri::Authority, client};
    

    pub struct HttpClient {
        scheme: String,
        authority: String,
    }

    impl HttpClient {

        /// Sends a GET request to the API as a means of checking if a connection can be made.
        pub async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let client = Client::new();
            client.get(Uri::from_static("http://127.0.0.1:7878")).await?;

            Ok(())
        }

        /// Send a POST request to the API for adding a new order.
        pub async fn add_order(&self, table_id: i32, item: &str)
        -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            let uri = Uri::builder()
                .scheme(self.scheme.as_str())
                .authority(self.authority.as_str())
                .path_and_query(format!("/orders?table_id={}&item={}", table_id, item))
                .build().unwrap();
            let client = Client::new();

            // Add parameters to body eventually
            let req = Request::builder()
                .method(Method::POST)
                .uri(uri)
                .body(Body::empty()).unwrap();

            let mut response = client.request(req).await?;
            Ok(String::from_utf8(body::to_bytes(response.body_mut()).await?.to_vec()).unwrap())
        }

        /// Sends a DELETE request to the API for deleting an order.
        pub async fn delete_order(&self, id: i32)
        -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            let uri = Uri::builder()
                .scheme(self.scheme.as_str())
                .authority(self.authority.as_str())
                .path_and_query(format!("/orders?id={}", id))
                .build().unwrap();
            let client = Client::new();
            
            // Add parameters to body eventually
            let req = Request::builder()
                .method(Method::DELETE)
                .uri(uri)
                .body(Body::empty()).unwrap();

            let mut response = client.request(req).await?;
            Ok(String::from_utf8(body::to_bytes(response.body_mut()).await?.to_vec()).unwrap())
        }

        /// Sends a GET request to the API to retrieve the orders of a given table id that have not finished preparations yet.
        pub async fn get_remaining_table_orders(&self, table_id: i32) 
        -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            let uri = Uri::builder()
                .scheme(self.scheme.as_str())
                .authority(self.authority.as_str())
                .path_and_query(format!("/tables/orders/remaining?table_id={}", table_id))
                .build().unwrap();
            let client = Client::new();

            let mut response = client.get(uri).await?;
            Ok(String::from_utf8(body::to_bytes(response.body_mut()).await?.to_vec()).unwrap())
        }

        /// Sends a GET request getting all orders for a table matching an item.
        pub async fn get_items_for_table(&self, table_id: i32, item: &str) 
        -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            let uri = Uri::builder()
                .scheme(self.scheme.as_str())
                .authority(self.authority.as_str())
                .path_and_query(format!("/tables/orders?table_id={}&item={}", table_id, item))
                .build().unwrap();
            let client = Client::new();
            let mut response = client.get(uri).await?;
            Ok(String::from_utf8(body::to_bytes(response.body_mut()).await?.to_vec()).unwrap())
        }

        pub async fn get_orders(&self)
        -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            let uri = Uri::builder()
                .scheme(self.scheme.as_str())
                .authority(self.authority.as_str())
                .path_and_query("/orders")
                .build().unwrap();
                let client = Client::new();
                let mut response = client.get(uri).await?;
                Ok(String::from_utf8(body::to_bytes(response.body_mut()).await?.to_vec()).unwrap())
        }

        pub fn new(scheme: &str, authority: &str) -> HttpClient {
            HttpClient {
                scheme: scheme.to_string(),
                authority: authority.to_string(),
            }
        } 
    }
}