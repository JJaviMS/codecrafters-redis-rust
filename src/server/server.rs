use super::client_connection::ClientConnection;
use std::net::TcpListener;

pub(crate) struct Server {
    tcp: TcpListener,
}

impl Server {
    pub(crate) fn new(tcp: TcpListener) -> Self {
        return Server { tcp: tcp };
    }

    pub(crate) fn run(&self) {
        println!("Accepting connections");
        for stream in self.tcp.incoming() {
            match stream {
                Ok(stream) => {
                    let mut client = ClientConnection::new(stream);

                    let client_result = client.handle_client();

                    if let Err(err) = client_result {
                        println!("Error has ocurred with the client: {}", err);
                    } else {
                        println!("Client handled successfully");
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}
