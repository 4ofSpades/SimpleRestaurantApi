pub mod data_models {
    use chrono::{DateTime, Utc};


    #[derive(Debug)]
    pub struct Order {
        pub id: i32,
        pub table_id: i32,
        pub created_at: DateTime<Utc>,
        pub item: String,
        pub finished_at: DateTime<Utc>,
    }

    impl Order {
        pub fn to_string(&self) -> String {
            let id_column_index = Order::get_id_column_index();
            let table_id_column_index = Order::get_table_id_column_index();
            let created_at_column_index = Order::get_created_at_column_index();
            let item_column_index = Order::get_item_column_index();
            let finished_at_column_index = Order::get_finished_at_column_index();

            format!("{}:{} {}:{} {}:{} {}:{} {}:{}", 
            id_column_index, self.id, 
            table_id_column_index, self.table_id,
            created_at_column_index, self.created_at,
            item_column_index, self.item,
            finished_at_column_index, self.finished_at)
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

        pub fn get_finished_at_column_index() -> usize {
            4
        }
    }
}