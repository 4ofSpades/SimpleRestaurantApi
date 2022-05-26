pub mod storage {
    use bb8::{Pool, RunError};
    use bb8_postgres::PostgresConnectionManager;
    use hyper::{Request, Body, Response, Method, StatusCode};
    use rand::{rngs::StdRng, SeedableRng, Rng};
    use tokio_postgres::{NoTls, Row};
    use std::{time::{SystemTime, UNIX_EPOCH}, collections::HashMap};
    use async_trait::async_trait;

    use crate::data_models::data_models::Order;

    /// Common interface providing headers for required functions. 
    #[async_trait]
    trait PostgresStorage {
        async fn add_order(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u32, item: String) -> Result<Response<Body>, RunError<tokio_postgres::Error>>;
        async fn delete_order(pool: Pool<PostgresConnectionManager<NoTls>>, id: u32) -> Result<Response<Body>, RunError<tokio_postgres::Error>>;
        async fn get_remaining_table_orders(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u32) -> Result<Response<Body>, RunError<tokio_postgres::Error>>;
        async fn get_items_for_table(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u32, items: &str) -> Result<Response<Body>, RunError<tokio_postgres::Error>>;
    }

    /// Type of storage that makes use of a relational database.
    struct PostgresDatabase ();

    /// Provides the Database struct with an implementation of the Storage trait.
    #[async_trait]
    impl PostgresStorage for PostgresDatabase {
        async fn add_order(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u32, item: String) 
        -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
            let mut rng = StdRng::from_entropy();
            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis().to_string();
            let now = now.parse::<i64>().unwrap();
            let duration_millis: u32 = rng.gen_range(15000..60000);
            let item = item.to_ascii_lowercase();
            let item = item.trim();

            let conn = pool.get().await?;
            let stmt = conn.prepare("INSERT INTO orders (table_id, created_at, item, duration) 
            VALUES ($1, $2, $3)").await?;
            let response = conn.execute(&stmt, &[&table_id, &now, &item, &duration_millis]).await?;
            Ok(Response::new(Body::from(format!("{} order of {} added for table {}", response, item, table_id))))
        }

        async fn delete_order(pool: Pool<PostgresConnectionManager<NoTls>>, id: u32)
        -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
            let conn = pool.get().await?;
            let stmt = conn.prepare("DELETE FROM orders
            WHERE id = $1").await?;
            let response = conn.execute(&stmt, &[&id]).await?;
            Ok(Response::new(Body::from(format!("{} order deleted with ID {}", response, id))))
        }

        async fn get_remaining_table_orders(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u32)
        -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis().to_string();
            let now = now.parse::<i64>().unwrap();
            let conn = pool.get().await?;
            let stmt = conn.prepare("SELECT * 
            FROM Orders 
            WHERE table_id = $1 
            AND created_at + duration < $2").await?;
            let response = conn.query(&stmt, &[&table_id, &now]).await?;

            let mut result: Vec<Order> = Vec::new();
            for row in response {
                result.push(row_to_order(&row));
            }

            let mut result_string = String::new();
            for order in result {
                result_string.push_str(format!("{}\n", order.to_string()).as_str())
            }
            Ok(Response::new(Body::from(result_string)))
        }

        async fn get_items_for_table(pool: Pool<PostgresConnectionManager<NoTls>>, table_id: u32, item: &str)
        -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
            let conn = pool.get().await?;
            let stmt = conn.prepare(
                "SELECT *
                FROM orders
                WHERE table_id = $1
                AND item LIKE $2").await?;
            let item = format!("%{}%", item);
            let response = conn.query(&stmt, &[&table_id, &item]).await?;
            let mut result: Vec<Order> = Vec::new();

            for row in response {
                result.push(row_to_order(&row));
            }

            let mut result_string: String = String::new();
            for order in result {
                result_string.push_str(format!("{}\n", order.to_string()).as_str());
            }

            Ok(Response::new(Body::from(result_string)))
        }
    }

    fn row_to_order(row: &Row) -> Order {
        let id: u32 = row.get(Order::get_id_column_index());
        let table_id: u32 = row.get(Order::get_table_id_column_index());
        // Will cause a problem in the future
        let created_at: i64 = row.get(Order::get_created_at_column_index());
        let item: String = row.get(Order::get_item_column_index());
        let duration: u32 = row.get(Order::get_duration_column_index());
        Order {
            id: id.try_into().unwrap(),
            table_id: table_id.try_into().unwrap(),
            created_at: created_at.try_into().unwrap(),
            item,
            duration: duration.try_into().unwrap(),
        }
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

            (&Method::GET, "/tables/orders") => {
                let params = convert_uri_query_to_hashmap(req.uri().query().unwrap());
                println!("Request received: Get orders containing {} for table {}", params["item"], params["table_id"]);
                *response.body_mut() = PostgresDatabase::get_items_for_table(pool.clone(), 
                params["table_id"].parse::<u32>().unwrap(), 
                params["item"]).await?.into_body();
            },

            (&Method::GET, "/tables/get-remaining-orders") => {
                let params = convert_uri_query_to_hashmap(req.uri().query().unwrap());
                println!("Request received: Get remaining items for table {}", params["table_id"]);
                *response.body_mut() = PostgresDatabase::get_remaining_table_orders(pool.clone(), params["table_id"].parse::<u32>().unwrap()).await?.into_body();
            },

            (&Method::POST, "/orders") => {
                let params = convert_uri_query_to_hashmap(req.uri().query().unwrap());
                println!("Request received: Add order of {} for table {}", params["item"], params["table_id"]);
                *response.body_mut() = PostgresDatabase::add_order(pool,
                    params["table_id"].parse::<u32>().unwrap(),
                    params["item"].to_string()).await?.into_body();
            },

            (&Method::DELETE, "/orders") => {
                let params = convert_uri_query_to_hashmap(req.uri().query().unwrap());
                println!("Request received: Delete order with ID {}", params["id"]);
                *response.body_mut() = PostgresDatabase::delete_order(pool.clone(), params["id"].parse::<u32>().unwrap()).await?.into_body();
            },


            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }

        Ok(response)
    }

    pub async fn init(pool: Pool<PostgresConnectionManager<NoTls>>) -> Result<Response<Body>, RunError<tokio_postgres::Error>> {
        let conn = pool.get().await?;
        let query = "BEGIN;
        DROP TABLE IF EXISTS orders;

        CREATE TABLE IF NOT EXISTS orders (
          id SERIAL PRIMARY KEY,
          table_id INTEGER NOT NULL,
          created_at VARCHAR NOT NULL,
          item VARCHAR(255) NOT NULL,
          duration int NOT NULL
        );

        INSERT INTO orders (table_id, created_at, item, duration) 
            VALUES (999, 500, 'Spaghetti', 100);
        COMMIT;";

        println!("Running init for DB");
        conn.batch_execute(query).await?;
        Ok(Response::new(Body::empty()))
    }

    fn convert_uri_query_to_hashmap(query: &str) -> HashMap<&str, &str> {
        query.split('&').map(|s| s.split_at(s.find("=").unwrap())).map(|(key, val)| (key, &val[1..])).collect()
    }
}