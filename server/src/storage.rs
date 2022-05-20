pub mod storage {
    use bb8::Pool;
    use bb8_postgres::PostgresConnectionManager;
    use hyper::{Request, Body, Response, Method, StatusCode};
    use rand::Rng;
    use tokio_postgres::NoTls;
    use { 
        std::{time::{SystemTime, UNIX_EPOCH}, sync::RwLock, thread, sync::Arc},
        async_trait::async_trait
    };

    /// Common interface providing headers for required functions. 
    #[async_trait]
    trait Storage {
        async fn add_order(table_id: u16, item: &str);
        async fn delete_order(table_id: u16, item: &str);
        async fn get_remaining_table_orders(table_id: u16) -> String;
        async fn get_items_for_table(table_id: u16, items: &str) -> String;
    }

    /// Type of storage that makes use of a relational database.
    struct Database ();

    /// Provides the Database struct with an implementation of the Storage trait.
    #[async_trait]
    impl Storage for Database {
        async fn add_order(table_id: u16, item: &str) {
            let mut rng = rand::thread_rng();
            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis();
            let sql = format!("INSERT INTO orders VALUES ({}, {}, {}, {})", table_id, now, item, rng.gen_range(5..15));
        }

        async fn delete_order(table_id: u16, item: &str) {
            let sql = "TODO";
        }

        async fn get_remaining_table_orders(table_id: u16) -> String {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis();
            let sql = format!("SELECT * 
            FROM Orders 
            WHERE table_id = {} 
            AND created_at + duration < {}", table_id, now);
            sql
        }

        async fn get_items_for_table(table_id: u16, item: &str) -> String {
            let sql = format!("SELECT *
            FROM Orders
            WHERE table_id = {}
            AND item LIKE '%{}%'", table_id, item);
            sql
        }
    }
    
    /// Handles HTTP request for the database implementation of the Storage trait.
    pub async fn handle_request_for_database(req: Request<Body>, pool: Pool<PostgresConnectionManager<NoTls>>) -> Result<Response<Body>, hyper::Error> {
        let mut response = Response::new(Body::empty());
        match(req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
                println!("Connection established");
                *response.status_mut() = StatusCode::OK; 
            },

            (&Method::GET, "/tables/get-items-for-table") => {
                //TODO: Validate request query 
                println!("Request received: Get items for table {}", 16);
                let result = Database::get_items_for_table(16, "Spaghetti").await;
                *response.body_mut() = Body::from(result);
            },

            (&Method::GET, "/tables/get-remaining-orders") => {
                let body = hyper::body::to_bytes(req.into_body()).await?;
                println!("Request received: Get remaining items for table {}", 16);
                let result = Database::get_remaining_table_orders(16).await;
                *response.body_mut() = Body::from(result);
            },

            (&Method::POST, "/orders") => {
                let body = hyper::body::to_bytes(req.into_body()).await?;
                println!("Request received: Add order {} for table {}","Spaghetti" ,16);
                let result = Database::add_order(16, "Spaghetti").await;
                //TODO: Return order id?
                *response.status_mut() = StatusCode::OK;
            },

            (&Method::DELETE, "/orders") => {
                let body = hyper::body::to_bytes(req.into_body()).await?;
                println!("Request received: Del order {} for table {}", "Spaghetti", 16);
                //TODO: Use order id?
                let result = Database::delete_order(16, "Spaghetti").await;
                *response.status_mut() = StatusCode::OK;
            },


            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }

        Ok(response)
    }
}