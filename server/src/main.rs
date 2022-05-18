use server::server::TcpServer;

mod data_models;
mod server;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let db_path = "/target/debug/db";
    let server = TcpServer { storage: storage::storage::Database::new(db_path)};
    server.run().await
}
