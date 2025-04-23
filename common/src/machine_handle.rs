use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MachineHandle {
    pub pool: String,
    pub index: usize,
}

impl MachineHandle {
    pub fn new(pool: String, index: usize) -> Self {
        MachineHandle { pool, index }
    }
}

impl ToString for MachineHandle {
    fn to_string(&self) -> String {
        format!("lxp-{}-{}", self.pool.to_string(), self.index)
    }
}
