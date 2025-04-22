use std::{thread, time::Duration};

use anyhow::Result;
use lxp_common::pool_definition::PoolDefinition;

use crate::lxd_machine::LxdMachine;

const EXEC_MAX_RETRIES: usize = 20;
const EXEC_TIMEOUT: Duration = Duration::from_secs(1);

pub struct PoolManager {
    pool_name: String,
    machines: Vec<LxdMachine>,
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
            PoolManager { pool_name: pool_definition.name.clone(), machines }
        )
    }
}
