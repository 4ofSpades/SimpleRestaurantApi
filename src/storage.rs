pub mod storage {
    use crate::data_models::data_models::Order;

    
    pub trait Storage {
        fn add_order(&self, order: Order);
        fn delete_order(&self, order_id: u16);
        fn get_remaining_table_orders(&self, table_id: u16);
    }

    pub struct Database();

    impl Storage for Database {
        fn add_order(&self, order: Order){

        }

        fn delete_order(&self, order_id: u16){

        }

        fn get_remaining_table_orders(&self, table_id: u16){

        }
    }
}