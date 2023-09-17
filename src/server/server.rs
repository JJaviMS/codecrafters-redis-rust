use crate::database::RedisDatabase;

use super::client_connection::ClientConnection;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

pub(crate) struct Server {
    tcp: TcpListener,
    database: Arc<RwLock<RedisDatabase>>,
}

impl Server {
    pub(crate) fn new(tcp: TcpListener) -> Self {
        let database = Arc::new(RwLock::new(RedisDatabase::new()));
        return Server {
            tcp: tcp,
            database: database,
        };
    }

    pub(crate) async fn run(&self) {
        println!("Accepting connections");
        loop {
            let stream = self.tcp.accept().await;

            println!("Client connection, yay!");
            match stream {
                Ok((stream, _)) => {
                    println!("Sending connection to thead");
                    let database = Arc::clone(&self.database);
                    tokio::spawn(async move {
                        let mut client = ClientConnection::new(stream);
                        println!("Executing client from a thread");
                        let client_result = client.handle_client(database).await;

                        if let Err(err) = client_result {
                            println!("Error has ocurred with the client: {}", err);
                        } else {
                            println!("Client handled successfully");
                        }
                    });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}
