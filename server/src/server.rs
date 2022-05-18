pub mod server {
    use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};
    use crate::storage::{storage::Storage, self};

    pub struct TcpServer<T: Storage>{
        pub storage: T,
    }

    impl<T: storage::storage::Storage> TcpServer<T> {
        pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
            let addr = "127.0.0.1:7878";
            let listener = TcpListener::bind(addr).await?;
            println!("Listening on {}", addr);
            loop {
                let (mut socket, _) = listener.accept().await?; 

                tokio::spawn(async move {
                    println!("Connection recieved!");
                    let mut buffer = vec![0; 1024];

                    loop {
                        let n = socket
                            .read(&mut buffer)
                            .await
                            .expect("failed to read data from socket");
        
                        if n == 0 {
                            return;
                        }
        
                        socket
                            .write_all(&buffer[0..n])
                            .await
                            .expect("failed to write data to socket");
                    }
                });
            }
        }
    }
}