pub mod server {
    use crate::storage::{storage::Storage, self};

    pub trait Server {
        fn add_order(&self);
        fn delete_order(&self);
        fn get_remaining_table_orders(&self);
    }

    pub struct TcpServer<T: Storage>{
        storage: T,
    }

    impl<T: storage::storage::Storage> Server for TcpServer<T> {
        fn add_order(&self) {
    
        } 
    
        fn delete_order(&self){
    
        }
    
        fn get_remaining_table_orders(&self){
        
        }
    }
}