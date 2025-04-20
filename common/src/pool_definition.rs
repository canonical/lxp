use std::{fs, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolDefinition {
    pub name: String,

    pub live_count: u32,
    pub pool_size: u32,

    pub base: String,

    pub prepare: Vec<String>,
}

impl PoolDefinition {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let raw: String = fs::read_to_string(path)?;
        let definition: PoolDefinition = serde_yaml::from_str(&raw)?;
        Ok(definition)
    }
}
