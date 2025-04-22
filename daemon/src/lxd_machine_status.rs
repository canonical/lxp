use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxdMachineStatus {
    Running,
    Stopped,
}

impl TryFrom<String> for LxdMachineStatus {
    type Error = anyhow::Error;

    fn try_from(status: String) -> Result<Self> {
        match status.as_str() {
            "Running" => Ok(LxdMachineStatus::Running),
            "Stopped" => Ok(LxdMachineStatus::Stopped),
            _ => bail!("Invalid status: {}", status),
        }
    }
}
