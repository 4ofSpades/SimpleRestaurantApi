pub mod storage {
    use rand::Rng;
    use crate::data_models::data_models::Order;
    use { 
        gluesql::{ prelude::{Glue, Payload, Value}, sled_storage::SledStorage },
        std::{time::{SystemTime, UNIX_EPOCH}, sync::RwLock, thread, sync::Arc},
        async_trait::async_trait
    };

    
    #[async_trait]
    pub trait Storage {
        async fn add_order(&self, table_id: u16, item: &str);
        async fn delete_order(&self, table_id: u16, item: &str);
        async fn get_remaining_table_orders(&self, table_id: u16);
        async fn get_items_for_table(&self, table_id: u16, items: &str);
    }

    pub struct Database {
        pub storage: SledStorage,
        order_increment: Arc<RwLock<u16>>,
    }

    #[async_trait]
    impl Storage for Database {
        async fn add_order(&self, table_id: u16, item: &str) {
            if let Ok(read_guard) = self.order_increment.read() {
                let mut rng = rand::thread_rng();
                let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis();
                let sql = format!("INSERT INTO orders VALUES ({}, {}, {}, {}, {})", *read_guard, table_id, now, item, rng.gen_range(5..15));
                let clone = self.storage.clone();

                let mut glue = Glue::new(clone);
                glue.execute(sql).unwrap();
            } 
            if let Ok(mut write_guard) = self.order_increment.write() {
                *write_guard += 1;
            }
        }

        async fn delete_order(&self, table_id: u16, item: &str) {
            if let Ok(_write_guard) = self.order_increment.write() {
                let sql = "TODO";
                let clone = self.storage.clone();
                let mut glue = Glue::new(clone);
                glue.execute(sql).unwrap();
            }
        }

        async fn get_remaining_table_orders(&self, table_id: u16) {
            //TODO: Use read_lock
            //TODO: Check if async is still useful
            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis();
            let sql = format!("SELECT * 
            FROM Orders 
            WHERE table_id = {} 
            AND created_at + duration < {}", table_id, now);
            let clone = self.storage.clone();
            let mut glue = Glue::new(clone);
            glue.execute(sql).unwrap();
        }

        async fn get_items_for_table(&self, table_id: u16, item: &str) {
            let sql = format!("SELECT *
            FROM Orders
            WHERE table_id = {}
            AND item LIKE '%{}%'", table_id, item);
            let clone = self.storage.clone();

            let mut glue = Glue::new(clone);
            glue.execute(sql).unwrap();
        }
    }

    impl Database {
        pub fn new(path: &str) -> Database {
            Database {
                storage: SledStorage::new(path).expect("Failed to create DB instance."),
                order_increment: Arc::new(RwLock::new(0)),
            }
        }

        pub fn init(&self) {
            let mut glue = Glue::new(self.storage.clone());
            let sql = "CREATE TABLE orders (
                id int,
                table_id int,
                created_at int,
                items varchar(255),
                duration int
              )";
            glue.execute(sql).unwrap();
        }
    }
}