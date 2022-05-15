use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Order {
    id: u16,
    table_id: u16,
    created_at: SystemTime
}

impl Order {
    
}