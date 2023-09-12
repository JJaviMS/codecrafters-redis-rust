use std::net::TcpListener;
use super::client_connection::ClientConnection;

pub(crate) struct Server {
    tcp: TcpListener
}

impl Server {

    
    pub(crate) fn new(tcp: TcpListener) -> Self {
        return Server { tcp: tcp }
    }

    
    pub(crate) fn run(&self) {
        println!("Accepting connections");
        for stream in self.tcp.incoming() {
            match stream {
                Ok(stream) => {
                    let mut client = ClientConnection::new(stream);

                    client.handle_client();
                    
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}