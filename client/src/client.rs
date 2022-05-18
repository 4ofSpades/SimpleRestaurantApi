pub mod client {
    use std::{net::SocketAddr, error::Error};
    use tokio::net::TcpStream;

    pub struct TcpClient();

    impl TcpClient {
        pub async fn connect(addr: &str) -> Result<(), Box<dyn Error>>{
            let socket = addr.parse::<SocketAddr>()?;
            let mut tcp_stream = TcpStream::connect(&socket).await?;
            println!("Connected to {}", addr);

            //TODO: Do some random API calls here
            Ok(())
        }
    }
}