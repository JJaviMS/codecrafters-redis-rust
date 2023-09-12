use std::io::{Read, Write};
use std::net::TcpStream;
use thiserror::Error;

use crate::resp::request_command::RequestCommandError;

use super::super::resp::request_command::RequestCommand;

#[derive(Debug, Error)]
pub(crate) enum ClientError {
    #[error("connection error")]
    ConnectionError(#[from] std::io::Error),

    #[error("Invalid command from the client")]
    ParseCommandError(#[from] RequestCommandError),
}

pub(crate) struct ClientConnection {
    client_stream: TcpStream,
}

impl ClientConnection {
    pub(crate) fn new(tcp_stream: TcpStream) -> Self {
        return ClientConnection {
            client_stream: tcp_stream,
        };
    }

    pub(crate) fn handle_client(&mut self) -> Result<(), ClientError> {
        let mut read_buffer: [u8; 1024] = [0; 1024];
        loop {
            println!("Reading from the client");
            let read_bytes = self.client_stream.read(&mut read_buffer)?;

            if read_bytes == 0 || read_buffer[0] == b'\0'{
                println!("Connection closed by the client");
                return Result::Ok(());
            }

            let read_command = std::str::from_utf8(&read_buffer[..read_bytes]).unwrap();

            println!("Read from client: {}", read_command);

            for line in read_command.lines() {
                let command = RequestCommand::try_from(line);
                if let Ok(command) = command  {
                    println!("Received correct command");
                    command.handle_command(self)
                } else {
                    println!("Received invalid command");
                }
            }

            

            

            
        }
    }

    pub(crate) fn send_to_client(&mut self, data: &str) {
        println!("Sending {} to the client", data);

        self.client_stream.write(data.as_bytes()).unwrap();
    }
}
