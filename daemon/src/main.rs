use std::collections::HashMap;

use anyhow::{anyhow, bail, Ok, Result};
use lxp_common::{machine_identifier::MachineIdentifier, pool_definition::PoolDefinition};
use lxp_daemon_connector::{daemon::LinuxPoolDaemon, message::Message, serve_target::ServeTarget};
use pool_manager::PoolManager;
use store::{list, retrieve, store};
use uuid::Uuid;

mod store;
mod pool_manager;
mod lxd_machine;
mod lxd_machine_status;

#[derive(Debug, PartialEq, Eq)]
enum NextAction {
    Continue,
    End,
}

fn handle_root(root_daemon: &mut LinuxPoolDaemon, pool_managers: &mut HashMap<String, PoolManager>) -> Result<()> {
    let message = root_daemon.listen_for_message()?;

    match message {
        Message::Initiate => {
            let daemon_file: String = Uuid::new_v4().to_string();
            let daemon_serve_target: ServeTarget = ServeTarget::Client(daemon_file);

            root_daemon.send_message(&Message::Begin(daemon_serve_target.clone()))?;
            let mut client_daemon: LinuxPoolDaemon = LinuxPoolDaemon::serve(daemon_serve_target)?;

            handle_client(&mut client_daemon, pool_managers)?;
        },
        _ => bail!("First message must be \"Initiate\""),
    }

    Ok(())
}

fn handle_client(client_daemon: &mut LinuxPoolDaemon, pool_managers: &mut HashMap<String, PoolManager>) -> Result<()> {
    loop {
        let message: Result<Message> = client_daemon.listen_for_message();

        if let Err(_) = message {
            return Ok(());
        }

        let message: Message = message.unwrap();
    
        let next_action: NextAction = match message.clone() {
            Message::DefinePool(pool_definition) => {
                store(&pool_definition.name, "pool-definitions", &pool_definition)?;
                Ok(NextAction::Continue)
            },
            Message::ListPools => {
                let pools: Vec<PoolDefinition> = list("pool-definitions")?;
                client_daemon.send_message(&Message::ListPoolsResponse(pools))?;
                Ok(NextAction::Continue)
            },
            Message::GetPool(name) => {
                let pool: PoolDefinition = retrieve(&name, "pool-definitions")?;
                client_daemon.send_message(&Message::GetPoolResponse(pool))?;
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

fn manifest_pools() -> Result<HashMap<String, PoolManager>> {
    let mut pool_managers: HashMap<String, PoolManager> = HashMap::new();

    let pools: Vec<PoolDefinition> = list("pool-definitions")?;
    for pool in pools {
        pool_managers.insert(pool.name.clone(), PoolManager::new(pool)?);
    }

    Ok(pool_managers)
}

fn main() -> Result<()> {
    let mut pool_managers: HashMap<String, PoolManager> = manifest_pools()?;

    loop {
        let mut root_daemon: LinuxPoolDaemon = LinuxPoolDaemon::serve(ServeTarget::Root)?;
        handle_root(&mut root_daemon, &mut pool_managers)?;
    }
}
