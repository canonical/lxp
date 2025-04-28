use std::fmt::Display;

use colorize::AnsiColor;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub enum LxpUnitStatus {
    Starting,
    Preparing,
    Ready,
    InUse,
    Stopping,
    Sleeping,
    Rebuilding,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct LxpPoolStatus {
    #[tabled(rename = "Pool")]
    pub name: String,
    #[tabled(rename = "Available units")]
    pub available_units: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct LxpMachineStatus {
    #[tabled(rename = "Unit")]
    pub name: String,
    #[tabled(rename = "Status")]
    pub status: LxpUnitStatus,
    #[tabled(rename = "Message")]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LxpStatus {
    pub pools: Vec<LxpPoolStatus>,
    pub units: Vec<LxpMachineStatus>,
}

impl Display for LxpUnitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified_status: String = match self {
            LxpUnitStatus::Starting => "Starting".to_string(),
            LxpUnitStatus::Preparing => "Preparing".to_string(),
            LxpUnitStatus::Ready => "Ready".green(),
            LxpUnitStatus::InUse => "In use".yellow(),
            LxpUnitStatus::Stopping => "Stopping".yellow(),
            LxpUnitStatus::Sleeping => "Sleeping".red(),
            LxpUnitStatus::Rebuilding => "Rebuilding".yellow(),
        };

        f.write_str(stringified_status.as_str())
    }
}

impl LxpPoolStatus {
    pub fn new(name: String, available_units: String) -> Self {
        LxpPoolStatus { name, available_units }
    }
}

impl LxpMachineStatus {
    pub fn new(name: String, status: LxpUnitStatus, message: String) -> Self {
        LxpMachineStatus { name, status, message }
    }
}

impl LxpStatus {
    pub fn new(pools: Vec<LxpPoolStatus>, units: Vec<LxpMachineStatus>) -> Self {
        LxpStatus { pools, units }
    }
}
