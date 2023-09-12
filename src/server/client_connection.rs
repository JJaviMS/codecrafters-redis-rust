use std::io::{Read, Write};
use std::net::TcpStream;
use anyhow::Ok;
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
            let read_bytes = self.client_stream.read(&mut read_buffer)?;
            let read_command = std::str::from_utf8(&read_buffer[..read_bytes]).unwrap();

            if read_buffer[0] == b'\0'{
                return Result::Ok(());
            }

            let command = RequestCommand::try_from(read_command)?;

            command.handle_command(self)
        }
    }

    pub(crate) fn send_to_client(&mut self, data: &str) {
        println!("Sending {} to the client", data);

        self.client_stream.write(data.as_bytes()).unwrap();
    }
}
