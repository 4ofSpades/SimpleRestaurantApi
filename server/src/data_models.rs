pub mod data_models {

    #[derive(Debug)]
    pub struct Order {
        pub id: u32,
        pub table_id: u32,
        pub created_at: u128,
        pub item: String,
        pub duration: u32,
    }

    impl Order {
        pub fn to_string(&self) -> String {
            let id_column_index = Order::get_id_column_index();
            let table_id_column_index = Order::get_table_id_column_index();
            let created_at_column_index = Order::get_created_at_column_index();
            let item_column_index = Order::get_item_column_index();
            let duration_column_index = Order::get_duration_column_index();

            format!("{}:{} {}:{} {}:{} {}:{} {}:{}", 
            id_column_index, self.id, 
            table_id_column_index, self.table_id,
            created_at_column_index, self.created_at,
            item_column_index, self.item,
            duration_column_index, self.duration)
        }

        pub fn get_id_column_index() -> usize {
            0
        }

        pub fn get_table_id_column_index() -> usize {
            1
        }

        pub fn get_created_at_column_index() -> usize {
            2
        }

        pub fn get_item_column_index() -> usize {
            3
        }

        pub fn get_duration_column_index() -> usize {
            4
        }
    }
}