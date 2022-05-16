pub mod server {
    use std::{net::TcpListener, thread};

    use crate::storage::{storage::Storage, self};

    pub trait Server {
        fn run(&self);
    }

    pub struct TcpServer<T: Storage>{
        storage: T,
    }

    impl<T: storage::storage::Storage> Server for TcpServer<T> {
        fn run(&self) {
            let port = "7878";
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("New connection: {}", stream.peer_addr().unwrap());
                        thread::spawn(move|| {
                            //DO something with client
                        });
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            drop(listener);
        }
    }
}