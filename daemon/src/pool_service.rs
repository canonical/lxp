use std::{collections::HashMap, sync::{Arc, Mutex}};

use anyhow::Result;
use lxp_common::pool_definition::PoolDefinition;
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
}
