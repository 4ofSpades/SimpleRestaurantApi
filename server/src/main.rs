use server::server::HttpServer;

mod data_models;
mod server;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpServer::run()
}
