use anyhow::{anyhow, Ok};
use lxp_common::pool_identifier::PoolIdentifier;

use crate::message::Message;

impl From<PoolIdentifier> for Message {
    fn from(message: PoolIdentifier) -> Self {
        Message::GrabMachine(message)
    }
}

impl TryFrom<Message> for PoolIdentifier {
    type Error = anyhow::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::GrabMachine(message) => Ok(message),
            _ => Err(anyhow!("Could not parse message for grab-machine command"))
        }
    }
}
