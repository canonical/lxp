use std::{thread::sleep, time::Duration};

use anyhow::{bail, Result};

use lxp_common::{machine_handle::MachineHandle, pool_definition::PoolDefinition};

use crate::{daemon::LinuxPoolDaemon, message::Message, serve_target::ServeTarget};

pub struct LinuxPoolConnector {
    daemon: LinuxPoolDaemon,
}

impl LinuxPoolConnector {
    pub fn connect() -> Result<Self> {
        let mut root_daemon: LinuxPoolDaemon = LinuxPoolDaemon::connect(ServeTarget::Root)?;
        let client_daemon_serve_target: ServeTarget = root_daemon.send_request(&Message::Initiate)?.try_into()?;
        sleep(Duration::from_millis(500));

        Ok(
            LinuxPoolConnector {
                daemon: LinuxPoolDaemon::connect(client_daemon_serve_target)?
            }
        )
    }

    pub fn define_pool(&mut self, pool_definition: PoolDefinition) -> Result<()> {
        self.daemon.send_message(&Message::DefinePool(pool_definition))
    }

    pub fn list_pools(&mut self) -> Result<Vec<PoolDefinition>> {
        match self.daemon.send_request(&Message::ListPools)? {
            Message::ListPoolsResponse(pools) => Ok(pools),
            Message::Error(error) => bail!("{}", error),
            _ => bail!("Could not list pools"),
        }
    }

    pub fn get_pool(&mut self, name: String) -> Result<PoolDefinition> {
        match self.daemon.send_request(&Message::GetPool(name))? {
            Message::GetPoolResponse(pool) => Ok(pool),
            Message::Error(error) => bail!("{}", error),
            _ => bail!("Could not get pool"),
        }
    }

    pub fn grab_machine(&mut self, pool: String) -> Result<MachineHandle> {
        match self.daemon.send_request(&Message::GrabMachine(pool))? {
            Message::GrabMachineResponse(machine) => Ok(machine),
            Message::Error(error) => bail!("{}", error),
            _ => bail!("Could not grab pool"),
        }
    }

    pub fn execute_command(&mut self, machine_handle: MachineHandle, command: String) -> Result<String> {
        match self.daemon.send_request(&Message::ExecuteCommand(machine_handle, command))? {
            Message::ExecuteCommandResponse(response) => Ok(response),
            Message::Error(error) => bail!("{}", error),
            _ => bail!("Could not execute command"),
        }
    }

    pub fn release_machine(&mut self, machine: MachineHandle) -> Result<()> {
        self.daemon.send_message(&Message::ReleaseMachine(machine))
    }
}
