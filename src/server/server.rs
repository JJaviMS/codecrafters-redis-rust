use super::client_connection::ClientConnection;
use tokio::net::TcpListener;

pub(crate) struct Server {
    tcp: TcpListener,
}

impl Server {
    pub(crate) fn new(tcp: TcpListener) -> Self {
        return Server { tcp: tcp };
    }

    pub(crate) async fn run(&self) {
        println!("Accepting connections");
        loop {
            let stream = self.tcp.accept().await;

            println!("Client connection, yay!");
            match stream {
                Ok((stream, _)) => {
                    let mut client = ClientConnection::new(stream);
                    println!("Sending connection to thead");
                    tokio::spawn(async move {
                        println!("Executing client from a thread");
                        let client_result = client.handle_client();

                        if let Err(err) = client_result.await {
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
