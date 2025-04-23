use anyhow::Result;
use clap::Parser;
use lxp_common::machine_handle::MachineHandle;
use lxp_daemon_connector::connector::LinuxPoolConnector;

#[derive(Debug, Parser)]
pub struct GrabMachineArgs {
    #[arg(value_name = "POOL")]
    pool_name: String,
}

pub fn run_command_grab_machine(lxp: &mut LinuxPoolConnector, args: GrabMachineArgs) -> Result<()> {
    let machine: MachineHandle = lxp.grab_machine(args.pool_name)?;

    println!("[{}]", machine.to_string());

    loop {
        break;
    }

    lxp.release_machine(machine)?;

    Ok(())
}
