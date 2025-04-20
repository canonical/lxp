use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::pool_identifier::PoolIdentifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineIdentifier {
    pool: PoolIdentifier,
    uuid: String,
}

impl MachineIdentifier {
    pub fn new(pool: &PoolIdentifier) -> Self {
        MachineIdentifier {
            pool: pool.clone(),
            uuid: Uuid::new_v4().to_string(),
        }
    }

    pub fn from(pool: PoolIdentifier, uuid: String) -> Self {
        MachineIdentifier { pool, uuid }
    }
}

impl ToString for MachineIdentifier {
    fn to_string(&self) -> String {
        format!("{}:{}", self.pool.to_string(), self.uuid)
    }
}
