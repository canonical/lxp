use std::{collections::HashMap, sync::{Arc, Mutex}};

use anyhow::{bail, Result};
use lxp_common::{machine_handle::MachineHandle, pool_definition::PoolDefinition};
use tokio::runtime::Runtime;

use crate::{pool_manager::PoolManager, store};

pub struct PoolService {
    pool_managers: Arc<Mutex<HashMap<String, PoolManager>>>,
    runtime: Runtime,
}

impl PoolService {
    pub fn new() -> Result<Self> {
        Ok(
            PoolService {
                pool_managers: Arc::new(Mutex::new(HashMap::new())),
                runtime: Runtime::new()?,
            }
        )
    }

    pub fn manifest_pools(&self) {
        let pool_managers: Arc<Mutex<HashMap<String, PoolManager>>> = Arc::clone(&self.pool_managers);

        self.runtime.spawn(async move {
            let pools: Vec<PoolDefinition> = store::list("pool-definitions").unwrap_or_default();
    
            for pool in pools {
                let name: String = pool.name.clone();
                match PoolManager::new(pool) {
                    Ok(pool_manager) => {
                        pool_managers.lock().unwrap().insert(name, pool_manager);
                    },
                    Err(error) => {
                        eprintln!("Failed to create pool {}: {}", name, error);
                    },
                }
            }
        });
    }

    pub fn grab_machine(&self, pool: &String) -> Result<MachineHandle> {
        match self.pool_managers.lock().unwrap().get_mut(pool) {
            Some(pool_manager) => {
                let machine: MachineHandle = pool_manager.grab_machine()?;
                return Ok(machine);
            },
            None => {
                bail!("Could not find pool \"{}\", make sure pool exists and daemon has completed pool manifestation", pool);
            },
        }
    }

    pub fn execute_command(&self, machine_handle: &MachineHandle, command: &String) -> Result<String> {
        match self.pool_managers.lock().unwrap().get(&machine_handle.pool) {
            Some(pool_manager) => {
                let response: String = pool_manager.execute_command(machine_handle, command)?;
                return Ok(response);
            },
            None => {
                bail!("Could not execute command");
            },
        }
    }

    pub fn release_machine(&self, machine_handle: &MachineHandle) -> Result<()> {
        match self.pool_managers.lock().unwrap().get_mut(&machine_handle.pool) {
            Some(pool_manager) => {
                pool_manager.release_machine(machine_handle)?;
                return Ok(());
            },
            None => {
                bail!("Could not find pool \"{}\", make sure pool exists and daemon has completed pool manifestation", machine_handle.pool);
            },
        }
    }
}
