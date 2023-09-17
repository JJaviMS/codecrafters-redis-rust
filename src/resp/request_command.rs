use super::{super::server::client_connection::ClientConnection, frames::Frame};
use thiserror::Error;

#[derive(Debug)]
pub(crate) enum RequestCommand {
    Ping,
    Echo(String),
}

#[derive(Debug, Error)]

pub(crate) enum RequestCommandError {
    #[error("Error parsing frames to commands")]
    ParseFramesError,

    #[error("not supported or unknown command")]
    UnknowCommand,
}

impl TryFrom<Frame> for RequestCommand {
    type Error = RequestCommandError;

    fn try_from(value: Frame) -> Result<Self, Self::Error> {
        if let Frame::Array(value) = value {
            if value.len() == 0 {
                return Err(RequestCommandError::ParseFramesError);
            }

            if let Frame::BulkString(command) = &value[0] {
                let command: Self = match command.to_ascii_lowercase().trim() {
                    "ping" => Self::Ping,
                    "echo" => {
                        if value.len() < 2 {
                            return Err(RequestCommandError::ParseFramesError);
                        }

                        let echo_string: &String;

                        if let Frame::BulkString(echo) = &value[1] {
                            echo_string = echo;
                        } else if let Frame::SimpleString(echo) = &value[1] {
                            echo_string = echo;
                        } else {
                            return Err(RequestCommandError::ParseFramesError);
                        }

                        Self::Echo(echo_string.to_owned())
                    }

                    _ => return Err(RequestCommandError::UnknowCommand),
                };
                return Ok(command);
            } else {
                return Err(RequestCommandError::ParseFramesError);
            }
        } else {
            return Err(RequestCommandError::ParseFramesError);
        }
    }
}

impl RequestCommand {
    pub(crate) async fn handle_command(&self, client: &mut ClientConnection) {
        match self {
            Self::Ping => handle_ping(client).await,
            Self::Echo(response) => handle_echo(client, response).await,
        }
    }
}

async fn handle_ping(client: &mut ClientConnection) {
    println!("Answering to ping");

    client.send_to_client("+PONG\r\n").await;
}

async fn handle_echo(client: &mut ClientConnection, response: &str) {
    println!("Answering to echo");

    let frame_answer = Frame::SimpleString(response.to_owned());


    client.send_to_client(&frame_answer.to_string()).await;
}
