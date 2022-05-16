pub mod storage {
    use std::{time::{SystemTime, UNIX_EPOCH}, thread};
    use rand::Rng;
    use crate::data_models::data_models::Order;
    use { 
        gluesql::{ prelude::{Glue, Payload, Value}, sled_storage::SledStorage },
        std::fs,
    };

    
    pub trait Storage {
        fn add_order(&self, table_id: u16, items: &str);
        fn delete_order(&self, order_id: u16);
        fn get_remaining_table_orders(&self, table_id: u16);
    }

    pub struct Database {
        storage: SledStorage,
    }

    impl Storage for Database {
        fn add_order(&self, table_id: u16, items: & str) {
            let mut rng = rand::thread_rng();
            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis();
            let sql = format!("INSERT INTO orders VALUES ({}, {}, {}, {})", table_id, now, items, rng.gen_range(5..15));
            let clone = self.storage.clone();

            thread::spawn(move || {
                let mut glue = Glue::new(clone);
                glue.execute(sql).unwrap();
            }).join().expect("Failed to add order.");
        }

        fn delete_order(&self, order_id: u16) {
            let sql = "TODO";
            let clone = self.storage.clone();

            thread::spawn(move || {
                let mut glue = Glue::new(clone);
                glue.execute(sql).unwrap();
            }).join().expect("Failed to delete order.");
            
        }

        fn get_remaining_table_orders(&self, table_id: u16) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis();
            let sql = format!("SELECT * 
            FROM Orders 
            WHERE table_id = {} 
            AND created_at + duration < {}", table_id, now);
            let clone = self.storage.clone();

            thread::spawn(move || {
                let mut glue = Glue::new(clone);
                glue.execute(sql).unwrap();
            }).join().expect("Failed to get remaining tables.");
        }
    }

    impl Database {
        fn new(path: &str) -> Database {
            Database {
                storage: SledStorage::new(path).expect("Failed to create DB instance."),
            }
        }

        fn init(&self) {
            let mut glue = Glue::new(self.storage.clone());
            let sql = fs::read_to_string("SimpleRestaurantApiDBDemo.sql").expect("Error reading the SQL file.");
            glue.execute(sql).unwrap();
        }
    }
}