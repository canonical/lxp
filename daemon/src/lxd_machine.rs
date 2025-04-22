use std::{process::Command, thread, time::Duration};

use anyhow::{anyhow, bail, Error, Result};
use lxp_common::machine_type::MachineType;

const EXEC_MAX_RETRIES: usize = 20;
const EXEC_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LxdMachine {
    machine_name: String,
    machine_type: MachineType,
    os_base: String,
}

impl LxdMachine {
    pub fn manifest(machine_name: String, machine_type: MachineType, os_base: String) -> Result<Self> {
        let machine: LxdMachine = LxdMachine { machine_name, machine_type, os_base };

        if machine.exists() {
            machine.delete()?;
        }

        machine.init()?;

        Ok(machine)
    }

    pub fn init(&self) -> Result<()> {
        let vm_argument: String = match &self.machine_type {
            MachineType::Container => String::new(),
            MachineType::VirtualMachine => "--vm".to_string(),
        };

        self.retry_run_host_shell(
            &format!(
                "lxc init {} {} {}",
                self.os_base,
                self.machine_name,
                vm_argument,
            ),
            EXEC_MAX_RETRIES,
            EXEC_TIMEOUT,
        )?;

        Ok(())
    }

    pub fn delete(&self) -> Result<()> {
        self.retry_run_host_shell(
            &format!(
                "lxc delete {} --force",
                self.machine_name,
            ),
            EXEC_MAX_RETRIES,
            EXEC_TIMEOUT,
        )?;

        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        self.retry_run_host_shell(
            &format!(
                "lxc start {}",
                self.machine_name,
            ),
            EXEC_MAX_RETRIES,
            EXEC_TIMEOUT,
        )?;

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        self.retry_run_host_shell(
            &format!(
                "lxc stop {}",
                self.machine_name,
            ),
            EXEC_MAX_RETRIES,
            EXEC_TIMEOUT,
        )?;

        Ok(())
    }

    pub fn exists(&self) -> bool {
        let lxc_machine: String = self.run_host_shell(
            &format!(
                "lxc list --format json | jq '.[] | .name' | grep \"^\\\"{}\\\"$\"",
                self.machine_name
            )
        )
            .unwrap_or_default()
            .trim()
            .to_string();

        lxc_machine == format!("\"{}\"", self.machine_name)
    }

    pub fn execute(&self, command: &String) -> Result<String> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "lxc exec {} -- bash << EOF\n{}\nEOF",
                self.machine_name,
                command
            ))
            .output()?;

        if output.status.success() {
            let output: String = String::from_utf8(output.stdout)?;
            Ok(output)
        }
        else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }

    pub fn retry_execute(&self, command: &String, max_retries: usize, timeout: Duration) -> Result<String> {
        let mut latest_error: Error = anyhow!("Failed to run \"{}\", but no error was found", command);

        for _ in 0..max_retries {
            match self.execute(&command) {
                Ok(output) => {
                    return Ok(output);
                },
                Err(error) => {
                    latest_error = error;
                },
            }
            thread::sleep(timeout);
        }

        Err(latest_error)
    }

    fn run_host_shell(&self, command: &String) -> Result<String> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(&command)
            .output()?;

        if output.status.success() {
            let output: String = String::from_utf8(output.stdout)?;
            Ok(output)
        }
        else {
            bail!(String::from_utf8(output.stderr).unwrap());
        }
    }

    fn retry_run_host_shell(&self, command: &String, max_retries: usize, timeout: Duration) -> Result<String> {
        let mut latest_error: Error = anyhow!("Failed to run \"{}\", but no error was found", command);

        for _ in 0..max_retries {
            match self.run_host_shell(command) {
                Ok(output) => {
                    return Ok(output);
                },
                Err(error) => {
                    latest_error = error;
                },
            }
            thread::sleep(timeout);
        }

        Err(latest_error)
    }
}
