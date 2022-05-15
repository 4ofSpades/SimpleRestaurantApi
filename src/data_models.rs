pub mod data_models {
    use rand::Rng;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug)]
    pub struct Item {
        id: u16,
        name: String,
        duration_minutes: u16,
    }

    impl Item {
        pub fn new(name: String) -> Item {
            let mut rng = rand::thread_rng();
            Item {
                id: 0, 
                name,
                duration_minutes: rng.gen_range(5..15),
            }
        }

        pub fn get_duration_in_millis(&self) -> u32 {
            (self.duration_minutes * 60000).into()
        } 
    }

    #[derive(Debug)]
    pub struct Order {
        id: u16,
        table_id: u16,
        created_at: u128,
        ordered_items: Vec<OrderedItems>,
    }

    impl Order {
        pub fn new(items: Vec<OrderedItems>) -> Order {
            Order {
                id: 0,
                table_id: 0,
                created_at: SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid time").as_millis(),
                ordered_items: items,
            }
        }
    }

    #[derive(Debug)]
    pub struct OrderedItems {
        order_id: u16,
        item_id: u16,
        quantity: u16,
    }

    impl OrderedItems {
        pub fn new(quantity: u16) -> OrderedItems {
            OrderedItems {
                order_id: 0,
                item_id: 0,
                quantity,
            }
        }
    }

    #[derive(Debug)]
    pub struct Restaurant {
        id: u16,
        name: String,
    }

    impl Restaurant {
        pub fn new(name: String) -> Restaurant {
            Restaurant {
                id: 0,
                name,
            }
        }
    }

    #[derive(Debug)]
    pub struct Waiter {
        id: u16,
        name: String,
        restaurant_id: u16,
    }

    impl Waiter {
        pub fn new(name: String) -> Waiter {
            Waiter {
                id: 0,
                name,
                restaurant_id: 0,
            }
        }
    }
}