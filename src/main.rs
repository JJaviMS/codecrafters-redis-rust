use std::net::TcpListener;
pub(crate) mod resp;
pub(crate) mod server;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    let server = server::server::Server::new(listener);

    server.run();
}
