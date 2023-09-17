use super::{
    super::server::client_connection::ClientConnection,
    frames::Frame,
};
use crate::database::RedisDatabase;
use std::{io, sync::Arc};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct SetOptions {
    key: String,
    value: String,
    expiration: Option<u64>,
}

#[derive(Debug)]
pub(crate) enum RequestCommand {
    Ping,
    Echo(String),
    Set(Box<SetOptions>),
    Get(String),
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

                        if let Some(s) = value[1].extract_string_from_frame() {
                            Self::Echo(s.to_owned())
                        } else {
                            return Err(RequestCommandError::ParseFramesError);
                        }
                    }
                    "set" => Self::Set(get_set_command(&value)?),
                    "get" => {
                        if value.len() < 2 {
                            return Err(RequestCommandError::ParseFramesError);
                        }

                        if let Some(s) = value[1].extract_string_from_frame() {
                            Self::Get(s.to_owned())
                        } else {
                            return Err(RequestCommandError::ParseFramesError);
                        }
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
    pub(crate) async fn handle_command(
        &self,
        client: &mut ClientConnection,
        database: &Arc<RwLock<RedisDatabase>>,
    ) -> io::Result<()> {
        return match self {
            Self::Ping => handle_ping(client).await,
            Self::Echo(response) => handle_echo(client, response).await,
            Self::Set(data) => handle_set(client, database, data).await,
            Self::Get(key) => handle_get(client, database, key).await,
        };
    }
}

async fn handle_ping(client: &mut ClientConnection) -> io::Result<()> {
    println!("Answering to ping");

    let frame_answer = Frame::SimpleString("PONG".to_owned());

    client.send_to_client(&frame_answer.to_string()).await
}

async fn handle_echo(client: &mut ClientConnection, response: &str) -> io::Result<()> {
    println!("Answering to echo");

    let frame_answer = Frame::SimpleString(response.to_owned());

    return client.send_to_client(&frame_answer.to_string()).await;
}

fn get_set_command(data: &[Frame]) -> Result<Box<SetOptions>, RequestCommandError> {
    let key = data[1]
        .extract_string_from_frame()
        .ok_or_else(|| RequestCommandError::ParseFramesError)?
        .to_owned();

    let value = data[2]
        .extract_string_from_frame()
        .ok_or_else(|| RequestCommandError::ParseFramesError)?
        .to_owned();

    let expiration: Option<u64> = if data.get(3).is_some_and(|frame| {
        frame
            .extract_string_from_frame()
            .is_some_and(|frame| frame == "PX")
    }) {
        data.get(4).and_then(|frame| {
            if let Frame::Integer(exp) = frame {
                Some(exp.to_owned())
            } else if let Some(str) = frame.extract_string_from_frame() {
                str.parse::<u64>().ok()
            } else {
                None
            }
        })
    } else {
        None
    };

    println!("Received set with a timeout: {:?}", expiration);

    return Ok(Box::new(SetOptions {
        key: key,
        value: value,
        expiration: expiration,
    }));
}

async fn handle_set(
    client: &mut ClientConnection,
    database: &Arc<RwLock<RedisDatabase>>,
    set: &SetOptions,
) -> io::Result<()> {
    let mut database = database.write().await;
    let _ = database.insert(&set.key, &set.value, set.expiration);

    let response = Frame::SimpleString("OK".to_owned());

    client.send_to_client(&response.to_string()).await
}

async fn handle_get(
    client: &mut ClientConnection,
    database: &Arc<RwLock<RedisDatabase>>,
    key: &str,
) -> io::Result<()> {
    let database = database.read().await;
    let value = database.get(key);

    let response = if let Some(value) = value {
        Frame::BulkString(value.to_owned())
    } else {
        Frame::Null
    };

    client.send_to_client(&response.to_string()).await
}
