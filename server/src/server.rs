pub mod server {
    use std::net::SocketAddr;

    use crate::storage::{storage::{handle_request_for_postgres_database, init}};
    use bb8::Pool;
    use bb8_postgres::PostgresConnectionManager;
    use hyper::{service::{make_service_fn, service_fn}, Server, Error, Response, Body};

    /// Defines a type for a HTTP server.
    pub struct HttpServer ();

    /// Provides an implementation for running an HTTP server.
    impl HttpServer {
        pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

            //TODO: Configure path using docker
            let pg_mgr = PostgresConnectionManager::new_from_stringlike(
                "postgres://postgres:postgrespw@localhost:55004",
                tokio_postgres::NoTls,
            )
            .unwrap();
        
            let pool = match Pool::builder().
            max_size(15).build(pg_mgr).await {
                Ok(pool) => pool,
                Err(e) => panic!("bb8 error {}", e),
            };

            println!("Pool built");

            init(pool.clone()).await?;

            println!("Database initialized!");
            
            let _ = Server::bind(&addr)
            .serve(make_service_fn(move |_| {
                let pool = pool.clone();
                async move {
                    Ok::<_, Error>(service_fn(move |request| {
                        let pool = pool.clone();
                        async move {
                            println!("Got request");
                            Ok::<_, Error>(match handle_request_for_postgres_database(request, pool).await {
                                Ok(rsp) => {
                                    println!("Sending success response");
                                    rsp
                                }
                                Err(e) => {
                                    println!("Sending error response");
                                    Response::new(Body::from(format!("Internal error {:?}", e)))
                                }
                        })
                    }
                }))
            }
        }))
        .await;
            Ok(())
        }
    }
}