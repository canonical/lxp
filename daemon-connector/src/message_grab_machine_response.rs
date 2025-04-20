use anyhow::{anyhow, Ok};
use lxp_common::machine_identifier::MachineIdentifier;

use crate::message::Message;

impl From<MachineIdentifier> for Message {
    fn from(message: MachineIdentifier) -> Self {
        Message::GrabMachineResponse(message)
    }
}

impl TryFrom<Message> for MachineIdentifier {
    type Error = anyhow::Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::GrabMachineResponse(message) => Ok(message),
            _ => Err(anyhow!("Could not parse response of grab-machine command"))
        }
    }
}
