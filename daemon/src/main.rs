use anyhow::{anyhow, bail, Ok, Result};
use lxp_common::machine_identifier::MachineIdentifier;
use lxp_daemon_connector::{daemon::LinuxPoolDaemon, message::Message, serve_target::ServeTarget};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
enum NextAction {
    Continue,
    End,
}

fn handle_root(root_daemon: &mut LinuxPoolDaemon) -> Result<()> {
    let message = root_daemon.listen_for_message()?;

    match message {
        Message::Initiate => {
            let daemon_file: String = Uuid::new_v4().to_string();
            let daemon_serve_target: ServeTarget = ServeTarget::Client(daemon_file);

            root_daemon.send_message(&Message::Begin(daemon_serve_target.clone()))?;
            let mut client_daemon: LinuxPoolDaemon = LinuxPoolDaemon::serve(daemon_serve_target)?;

            handle_client(&mut client_daemon)?;
        },
        _ => bail!("First message must be \"Initiate\""),
    }

    Ok(())
}

fn handle_client(client_daemon: &mut LinuxPoolDaemon) -> Result<()> {
    loop {
        let message = client_daemon.listen_for_message()?;
    
        let next_action = match message.clone() {
            Message::DefinePool(pool_definition) => {
                Ok(NextAction::Continue)
            },
            Message::GrabMachine(pool_identifier) => {
                let machine_identifier: MachineIdentifier = MachineIdentifier::new(&pool_identifier);
                client_daemon.send_message(&machine_identifier.into())?;
                Ok(NextAction::Continue)
            },
            Message::End => Ok(NextAction::End),
            _ => {
                Err(anyhow!("Invalid message sent to daemon"))
            }
        }?;

        if next_action == NextAction::End {
            return Ok(());
        }
    }
}

fn main() -> anyhow::Result<()> {    
    loop {
        let mut root_daemon: Box<LinuxPoolDaemon> = Box::new(LinuxPoolDaemon::serve(ServeTarget::Root)?);
        handle_root(&mut root_daemon)?;
    }
}
