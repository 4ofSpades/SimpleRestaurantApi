pub mod storage {
    use bb8::{Pool, RunError};
    use bb8_postgres::PostgresConnectionManager;
    use hyper::{Request, Body, Response, Method, StatusCode};
    use rand::{rngs::StdRng, SeedableRng, Rng};
    use tokio_postgres::NoTls;
    use { 
        std::time::{SystemTime, UNIX_EPOCH},
        async_trait::async_trait
    };

    /// Common interface providing headers for required functions. 
    #[async_trait]
    trait PostgresStorage {
        async fn add_order(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u16, item: String) -> Result<Response<Body>, RunError<tokio_postgres::Error>>;
        // async fn delete_order(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u16, item: &str) -> Result<Response<Body>, RunError<tokio_postgres::Error>>;
        // async fn get_remaining_table_orders(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u16) -> Result<Response<Body>, RunError<tokio_postgres::Error>>;
        // async fn get_items_for_table(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u16, items: &str) -> Result<Response<Body>, RunError<tokio_postgres::Error>>;
    }

    /// Type of storage that makes use of a relational database.
    struct PostgresDatabase ();

    /// Provides the Database struct with an implementation of the Storage trait.
    #[async_trait]
    impl PostgresStorage for PostgresDatabase {
        async fn add_order(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u16, item: String) 
        -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
            let mut rng = StdRng::from_entropy();
            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis();
            let duration_millis: u16 = rng.gen_range(15000..60000);
            let item = item.to_ascii_lowercase();
            let item = item.trim();
            
            let query = format!("BEGIN;
            INSERT INTO orders (table_id, created_at, item, duration) 
            VALUES ({}, {}, {}, {});
            COMMIT;",table_id, now, item, duration_millis);

            let conn = pool.get().await?;
            conn.query(&query, &[]).await?;
            Ok(Response::new(Body::from(format!("Order added"))))
        }

        // async fn delete_order(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u16, item: &str)
        // -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
        //     let sql = "TODO";
        // }

        // async fn get_remaining_table_orders(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u16)
        // -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
        //     let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis();
        //     let sql = format!("SELECT * 
        //     FROM Orders 
        //     WHERE table_id = {} 
        //     AND created_at + duration < {}", table_id, now);
        //     sql
        // }

        // async fn get_items_for_table(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u16, item: &str)
        // -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
        //     let sql = format!("SELECT *
        //     FROM Orders
        //     WHERE table_id = {}
        //     AND item LIKE '%{}%'", table_id, item);
        //     sql
        // }
    }
    
    /// Handles HTTP request for the postgres database implementation.
    pub async fn handle_request_for_postgres_database(req: Request<Body>, pool: Pool<PostgresConnectionManager<NoTls>>) 
    -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
        let mut response = Response::new(Body::empty());
        match(req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
                println!("Connection established");
                *response.status_mut() = StatusCode::OK; 
            },

            // (&Method::GET, "/tables/get-items-for-table") => {
            //     println!("Request received: Get items for table {}", 16);
            //     //TODO
            //     *response.body_mut() = PostgresDatabase::get_items_for_table(pool.clone(), 16, "Spaghetti").await?.into_body();
            // },

            // (&Method::GET, "/tables/get-remaining-orders") => {
            //     println!("Request received: Get remaining items for table {}", 16);
            //     *response.body_mut() = PostgresDatabase::get_remaining_table_orders(pool.clone(), 16).await?.into_body();
            // },

            (&Method::POST, "/orders") => {
                println!("Request received: Add order {} for table {}","Spaghetti" ,16);
                PostgresDatabase::add_order(pool, 16, "Spaghetti".to_string()).await?;
                *response.status_mut() = StatusCode::OK;
            },

            // (&Method::DELETE, "/orders") => {
            //     println!("Request received: Del order {} for table {}", "Spaghetti", 16);
            //     let result = PostgresDatabase::delete_order(pool.clone(), 16, "Spaghetti").await;
            //     *response.status_mut() = StatusCode::OK;
            // },


            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }

        Ok(response)
    }

    pub async fn init(pool: Pool<PostgresConnectionManager<NoTls>>) -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
        let conn = pool.get().await?;
        let query = "BEGIN;
        DROP TABLE orders;

        CREATE TABLE orders (
          id SERIAL PRIMARY KEY,
          table_id INTEGER NOT NULL,
          created_at INTEGER NOT NULL,
          item VARCHAR(255) NOT NULL,
          duration int NOT NULL
        );
        COMMIT;";
        conn.query(query, &[]).await?;
        Ok(Response::new(Body::empty()))
    }
}