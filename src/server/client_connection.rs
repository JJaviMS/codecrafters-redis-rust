use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write};
use super::super::resp::request_command::RequestCommand;

pub(crate) struct ClientConnection {
    client_stream: TcpStream
}


impl ClientConnection {
    pub(crate) fn new(tcp_stream: TcpStream) -> Self {
        return ClientConnection { client_stream: tcp_stream }
    }


    pub(crate) fn handle_client(&mut self){
        let buf_reader = BufReader::new(&self.client_stream);
        let line = buf_reader.lines().next().unwrap().unwrap();

        let command = RequestCommand::try_from(line.as_str());

        command.unwrap().handle_command(self);
        
    }

    pub(crate) fn send_to_client(&mut self, data: &str){
        println!("Sending {} to the client", data);

        self.client_stream.write(data.as_bytes()).unwrap();
    }
}