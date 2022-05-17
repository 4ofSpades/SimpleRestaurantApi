mod data_models;
mod server;
mod storage;

fn main() {
    let path = "/target/debug/db";
    let server = server::server::TcpServer { storage: storage::storage::Database::new(path) };
    server.storage.init();
    
    println!("Hello, world!");
}
