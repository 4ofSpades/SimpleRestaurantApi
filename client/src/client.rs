pub mod client {
    use hyper::{Client, Request, Method, Body, Uri, http::Error};
    

    pub struct HttpClient {
        base_addr: String,
    }

    impl HttpClient {

        /// Sends a GET request to the API as a means of checking if a connection can be made.
        pub async fn test_connection(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let client = Client::new();
            client.get(Uri::from_static("http://127.0.0.1:7878")).await?;

            Ok(())
        }

        /// Send a POST request to the API for adding a new order.
        pub async fn add_order(&self, table_id: u16, item: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            //let uri = Uri::builder().authority(self.base_addr.to_string() + "/orders").build().unwrap();
            let client = Client::new();
            let req = Request::builder()
                .method(Method::POST)
                .uri(self.base_addr.to_string() + "/orders")
                .header("", "")
                .body(Body::from("")).unwrap();

            client.request(req).await?;
            Ok(())
        }

        // async fn delete_order(&self, table_id: u16, item: &str) {
        //     let uri = self.base_addr.to_string() + "/orders";
        //     let client = Client::new();
        //     let req = Request::builder()
        //         .method(Method::POST)
        //         .uri(uri)
        //         .header("", "")
        //         .body(Body::from("")).unwrap();

        //     client.request(req).await;
        // }

        // async fn get_remaining_table_orders(&self, table_id: u16) -> String {
        //     let uri = self.base_addr.to_string() + "/orders";
        //     let client = Client::new();
        //     let req = Request::builder()
        //         .method(Method::POST)
        //         .uri(uri)
        //         .header("", "")
        //         .body(Body::from("")).unwrap();

        //     client.request(req).await?.body()
        // }

        // async fn get_items_for_table(&self, table_id: u16, items: &str) -> String {
        //     let uri = self.base_addr.to_string() + "/orders";
        //     let client = Client::new();
        //     let req = Request::builder()
        //         .method(Method::POST)
        //         .uri(uri)
        //         .header("", "")
        //         .body(Body::from("")).unwrap();

        //     client.request(req).await?.body();
        //     "AAA".to_string()
        // }

        pub fn new(base_addr: &str) -> HttpClient {
            HttpClient {
                base_addr: base_addr.to_string(),
            }
        } 
    }
}