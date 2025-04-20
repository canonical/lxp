use anyhow::{anyhow, Ok};

use crate::{message::Message, serve_target::ServeTarget};

impl From<ServeTarget> for Message {
    fn from(message: ServeTarget) -> Self {
        Message::Begin(message)
    }
}

impl TryFrom<Message> for ServeTarget {
    type Error = anyhow::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Begin(message) => Ok(message),
            _ => Err(anyhow!("Could not parse message for begin command"))
        }
    }
}
