use anyhow::{anyhow, bail, Ok, Result};
use lxp_common::{lxp_status::LxpStatus, machine_handle::MachineHandle, pool_definition::PoolDefinition};
use lxp_daemon_connector::{daemon::LinuxPoolDaemon, message::Message, serve_target::ServeTarget};
use pool_service::PoolService;
use store::{list, retrieve, store};
use uuid::Uuid;

mod store;
mod pool_manager;
mod pool_service;
mod lxd_machine;
mod lxd_machine_status;

#[derive(Debug, PartialEq, Eq)]
enum NextAction {
    Continue,
    End,
}

fn handle_root(root_daemon: &mut LinuxPoolDaemon, pool_service: &mut PoolService) -> Result<()> {
    let message = root_daemon.listen_for_message()?;

    match message {
        Message::Initiate => {
            let daemon_file: String = Uuid::new_v4().to_string();
            let daemon_serve_target: ServeTarget = ServeTarget::Client(daemon_file);

            root_daemon.send_message(&Message::Begin(daemon_serve_target.clone()))?;
            let mut client_daemon: LinuxPoolDaemon = LinuxPoolDaemon::serve(daemon_serve_target)?;

            if let Err(error) = handle_client(&mut client_daemon, pool_service) {
                client_daemon.send_message(&Message::Error(error.to_string()))?;
            }
        },
        _ => bail!("First message must be \"Initiate\""),
    }

    Ok(())
}

fn handle_client(client_daemon: &mut LinuxPoolDaemon, pool_service: &mut PoolService) -> Result<()> {
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
            Message::GrabMachine(pool) => {
                let machine: MachineHandle = pool_service.grab_machine(&pool)?;
                client_daemon.send_message(&Message::GrabMachineResponse(machine))?;
                Ok(NextAction::Continue)
            },
            Message::ExecuteCommand(machine_handle, command) => {
                let response: String = pool_service.execute_command(&machine_handle, &command)?;
                client_daemon.send_message(&Message::ExecuteCommandResponse(response))?;
                Ok(NextAction::Continue)
            },
            Message::ReleaseMachine(machine) => {
                pool_service.release_machine(&machine)?;
                Ok(NextAction::Continue)
            },
            Message::Status => {
                let status: LxpStatus = pool_service.status()?;
                client_daemon.send_message(&Message::StatusResponse(status))?;
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

#[tokio::main]
async fn main() -> Result<()> {
    let mut pool_service: PoolService = PoolService::new()?;

    pool_service.manifest_pools();

    loop {
        let mut root_daemon: LinuxPoolDaemon = LinuxPoolDaemon::serve(ServeTarget::Root)?;
        handle_root(&mut root_daemon, &mut pool_service)?;
    }
}
