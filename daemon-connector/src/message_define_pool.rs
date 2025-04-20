use anyhow::{anyhow, Ok};
use lxp_common::pool_definition::PoolDefinition;

use crate::message::Message;

impl From<PoolDefinition> for Message {
    fn from(message: PoolDefinition) -> Self {
        Message::DefinePool(message)
    }
}

impl TryFrom<Message> for PoolDefinition {
    type Error = anyhow::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::DefinePool(message) => Ok(message),
            _ => Err(anyhow!("Could not parse message for define-pool command"))
        }
    }
}
