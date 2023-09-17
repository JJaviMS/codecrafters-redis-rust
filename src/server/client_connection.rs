use std::io::Cursor;

use bytes::{Buf, BytesMut};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::resp::frames::{Frame, FrameParseError};
use crate::resp::request_command::{RequestCommand, RequestCommandError};

#[derive(Debug, Error)]
pub(crate) enum ClientError {
    #[error("connection error")]
    ConnectionError(#[from] std::io::Error),

    #[error("Invalid command from the client")]
    ParseCommandError(#[from] FrameParseError),

    #[error("The command could not be handled")]
    CommandError(#[from] RequestCommandError),

    #[error("connection reset")]
    ConnectionReset,

    #[error("not implemented")]
    NotImplemented,
}
#[derive(Debug)]
pub(crate) struct ClientConnection {
    client_stream: TcpStream,
    buffer: BytesMut,
}

impl ClientConnection {
    pub(crate) fn new(tcp_stream: TcpStream) -> Self {
        return ClientConnection {
            client_stream: tcp_stream,
            buffer: BytesMut::with_capacity(4096),
        };
    }

    pub(crate) async fn handle_client(&mut self) -> Result<(), ClientError> {
        loop {
            match self.read_frame().await {
                Ok(Some(frame)) => {
                    RequestCommand::try_from(frame)?.handle_command(self).await;
                }

                Err(ClientError::ConnectionReset) => {
                    return Ok(());
                }

                Ok(None) => {
                    return Ok(());
                }

                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    async fn read_frame(&mut self) -> Result<Option<Frame>, ClientError> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            println!("Reading from the client");

            let read_size = self.client_stream.read_buf(&mut self.buffer).await?;

            if read_size == 0 {
                println!("No read from the client");
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(ClientError::ConnectionReset);
                }
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>, FrameParseError> {
        if !self.buffer.has_remaining() {
            return Ok(None);
        }

        let mut buf = Cursor::new(&self.buffer[..]);

        let frame = Frame::parse_from_buf(&mut buf);

        match frame {
            Ok(frame) => {
                self.buffer.advance(buf.position() as usize);
                return Ok(Some(frame));
            }
            Err(FrameParseError::Incomplete) => {
                return Ok(None);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub(crate) async fn send_to_client(&mut self, data: &str) -> Result<(), std::io::Error> {
        println!("Sending {} to the client", data);

        return self.client_stream.write(data.as_bytes()).await.map(|_| ());
    }
}
