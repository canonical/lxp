use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PoolDefinition {
    pub pool_size: u32,
    pub live_count: u32,
}

impl PoolDefinition {
    pub fn new(pool_size: u32, live_count: u32) -> Self {
        PoolDefinition { pool_size, live_count }
    }
}
