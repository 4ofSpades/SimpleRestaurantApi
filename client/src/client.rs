pub mod client {
    use hyper::{Client, Request, Method, Body};
    

    pub struct HttpClient {
        base_addr: String,
    }

    impl HttpClient {
        async fn add_order(&self, table_id: u16, item: &str) {
            let uri = self.base_addr.to_string() + "/orders";
            let client = Client::new();
            let req = Request::builder()
                .method(Method::POST)
                .uri(uri)
                .header("", "")
                .body(Body::from("")).unwrap();

            client.request(req).await;
        }

        async fn delete_order(&self, table_id: u16, item: &str) {
            let uri = self.base_addr.to_string() + "/orders";
            let client = Client::new();
            let req = Request::builder()
                .method(Method::POST)
                .uri(uri)
                .header("", "")
                .body(Body::from("")).unwrap();

            client.request(req).await;
        }

        async fn get_remaining_table_orders(&self, table_id: u16) -> String {
            let uri = self.base_addr.to_string() + "/orders";
            let client = Client::new();
            let req = Request::builder()
                .method(Method::POST)
                .uri(uri)
                .header("", "")
                .body(Body::from("")).unwrap();

            client.request(req).await?.body()
        }

        async fn get_items_for_table(&self, table_id: u16, items: &str) -> String {
            let uri = self.base_addr.to_string() + "/orders";
            let client = Client::new();
            let req = Request::builder()
                .method(Method::POST)
                .uri(uri)
                .header("", "")
                .body(Body::from("")).unwrap();

            client.request(req).await?.body()
        }

        pub fn new(base_addr: &str) -> HttpClient {
            HttpClient {
                base_addr: base_addr.to_string(),
            }
        } 
    }
}