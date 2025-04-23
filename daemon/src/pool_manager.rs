use std::{collections::HashSet, time::Duration};

use anyhow::{bail, Result};
use lxp_common::{machine_handle::MachineHandle, pool_definition::PoolDefinition};

use crate::lxd_machine::LxdMachine;

const EXEC_MAX_RETRIES: usize = 20;
const EXEC_TIMEOUT: Duration = Duration::from_secs(1);

pub struct PoolManager {
    pool_definition: PoolDefinition,
    machines: Vec<LxdMachine>,
    machines_in_use: HashSet<MachineHandle>,
}

impl PoolManager {
    pub fn new(pool_definition: PoolDefinition) -> Result<Self> {
        let mut machines: Vec<LxdMachine> = Vec::new();

        for i in 0..pool_definition.pool_size {
            let machine: LxdMachine = LxdMachine::manifest(
                format!("lxp-{}-{}", pool_definition.name, i),
                pool_definition.machine_type.clone(),
                pool_definition.base.clone()
            )?;

            machine.start()?;

            for command in &pool_definition.prepare {
                machine.retry_execute(&command, EXEC_MAX_RETRIES, EXEC_TIMEOUT)?;
            }

            if i >= pool_definition.live_count {
                machine.stop()?;
            }

            machines.push(machine);
        }

        Ok(
            PoolManager {
                pool_definition,
                machines,
                machines_in_use: HashSet::new(),
            }
        )
    }

    pub fn grab_machine(&mut self) -> Result<MachineHandle> {
        let machine_handle: MachineHandle = self.find_next_available_machine()?;
        self.machines_in_use.insert(machine_handle.clone());

        Ok(machine_handle)
    }

    pub fn release_machine(&mut self, machine_handle: &MachineHandle) -> Result<()> {
        match self.machines.get(machine_handle.index) {
            Some(machine) => {
                machine.delete()?;
                machine.init()?;
                machine.start()?;
                
                for command in &self.pool_definition.prepare {
                    machine.retry_execute(command, EXEC_MAX_RETRIES, EXEC_TIMEOUT)?;
                }

                if self.nb_machines_available() > self.pool_definition.live_count as usize {
                    machine.stop()?;
                }
            },
            None => {
                bail!("Could not get machine \"{}\"", machine_handle.to_string());
            }
        }

        self.machines_in_use.remove(machine_handle);

        Ok(())
    }

    fn find_next_available_machine(&self) -> Result<MachineHandle> {
        for i in 0..self.machines.len() {
            let machine_handle: MachineHandle = MachineHandle::new(self.pool_definition.name.clone(), i);
            if !self.is_in_use(&machine_handle) {
                return Ok(machine_handle);
            }
        }
        bail!("No machine is available in this pool");
    }

    fn is_in_use(&self, machine: &MachineHandle) -> bool {
        self.machines_in_use.contains(machine)
    }

    fn nb_machines_available(&self) -> usize {
        self.machines.len() - self.machines_in_use.len()
    }
}
