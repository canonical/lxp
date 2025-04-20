use std::{thread::sleep, time::Duration};

use anyhow::{bail, Result};

use lxp_common::{machine_identifier::MachineIdentifier, pool_definition::PoolDefinition, pool_identifier::PoolIdentifier};

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
            _ => bail!("Could not list pools"),
        }
    }

    pub fn get_pool(&mut self, name: String) -> Result<PoolDefinition> {
        match self.daemon.send_request(&Message::GetPool(name))? {
            Message::GetPoolResponse(pool) => Ok(pool),
            _ => bail!("Could not get pool"),
        }
    }

    pub fn grab_machine(&mut self, pool_identifier: PoolIdentifier) -> Result<MachineIdentifier> {
        self.daemon.send_request(&Message::GrabMachine(pool_identifier))?.try_into()
    }
}
