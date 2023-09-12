use super::super::server::client_connection::ClientConnection;
use thiserror::Error;

#[derive(Debug)]
pub(crate) enum RequestCommand {
    Ping,
}

#[derive(Debug, Error)]

pub(crate) enum RequestCommandError {
    #[error("Error parsing the command: \"{0}\"")]
    ParseError(String),
}

impl TryFrom<&str> for RequestCommand {
    type Error = RequestCommandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        return match value.trim() {
            "PING" => Ok(Self::Ping),
            _ => Err(RequestCommandError::ParseError(value.to_string())),
        };
    }
}

impl RequestCommand {
    pub(crate) fn handle_command(&self, client: &mut ClientConnection) {
        match self {
            Self::Ping => handle_ping(client),
        }
    }
}

fn handle_ping(client: &mut ClientConnection) {
    println!("Answering to ping");

    client.send_to_client("+PONG\r\n");
}
