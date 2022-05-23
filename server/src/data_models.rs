pub mod data_models {

    #[derive(Debug)]
    pub struct Order {
        id: u16,
        table_id: u16,
        created_at: u128,
        item: String,
        duration: u16,
    }

    impl Order {
        pub fn new(id: u16, table_id: u16, created_at: u128, item: String, duration: u16) -> Order {
            Order {
                id,
                table_id,
                created_at,
                item,
                duration,
            }
        }

        pub fn to_string(&self) -> String {
            let id_name = Order::get_id_name();
            let table_id_name = Order::get_table_id_name();
            let created_at_name = Order::get_created_at_name();
            let item_name = Order::get_item_name();
            let duration_name = Order::get_duration_name();

            format!("{}:{} {}:{} {}:{} {}:{} {}:{}", 
            id_name, self.id, 
            table_id_name, self.table_id,
            created_at_name, self.created_at,
            item_name, self.item,
            duration_name, self.duration)
        }

        pub fn get_id_name() -> String {
            "id".to_string()
        }

        pub fn get_table_id_name() -> String {
            "table_id".to_string()
        }

        pub fn get_created_at_name() -> String {
            "created_at".to_string()
        }

        pub fn get_item_name() -> String {
            "item".to_string()
        }

        pub fn get_duration_name() -> String {
            "duration".to_string()
        }
    }
}