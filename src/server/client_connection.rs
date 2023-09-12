use std::net::TcpStream;
use std::io::{BufReader, Write, Read};
use super::super::resp::request_command::RequestCommand;

pub(crate) struct ClientConnection {
    client_stream: TcpStream
}


impl ClientConnection {
    pub(crate) fn new(tcp_stream: TcpStream) -> Self {
        return ClientConnection { client_stream: tcp_stream }
    }


    pub(crate) fn handle_client(&mut self){
        let mut buf_reader = BufReader::new(&self.client_stream);
        let mut read_buffer: [u8;1024] = [0;1024];
        let read_bytes = buf_reader.read(&mut read_buffer).unwrap();
        let read_command = std::str::from_utf8(&read_buffer[..read_bytes]).unwrap();

        let command = RequestCommand::try_from(read_command);

        command.unwrap().handle_command(self);
        
    }

    pub(crate) fn send_to_client(&mut self, data: &str){
        println!("Sending {} to the client", data);

        self.client_stream.write(data.as_bytes()).unwrap();
    }
}