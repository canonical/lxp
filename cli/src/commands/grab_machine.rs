use std::io::{stdin, stdout, Write};

use anyhow::Result;
use clap::Parser;
use colorize::AnsiColor;
use lxp_common::machine_handle::MachineHandle;
use lxp_daemon_connector::connector::LinuxPoolConnector;

#[derive(Debug, Parser)]
pub struct GrabMachineArgs {
    #[arg(value_name = "POOL")]
    pool_name: String,
}

pub fn run_command_grab_machine(lxp: &mut LinuxPoolConnector, args: GrabMachineArgs) -> Result<()> {
    let machine: MachineHandle = lxp.grab_machine(args.pool_name)?;

    println!("Grabbed machine {}", machine.clone().to_string().bold().green());

    loop {
        let machine: MachineHandle = machine.clone();
        let mut user_input: String = String::new();

        print!("{}{}{}", "┌──[".bold().blue(), machine.to_string(), "]\n└─› ".bold().blue());

        stdout().flush().ok();
        stdin().read_line(&mut user_input)?;

        if let Some('\n') = user_input.chars().next_back() {
            user_input.pop();
        }

        if let Some('\r') = user_input.chars().next_back() {
            user_input.pop();
        }

        if user_input == "exit" {
            break;
        }

        let response: String = lxp.execute_command(machine, user_input)?;
        print!("{}", response.italic());
    }

    lxp.release_machine(machine)?;

    Ok(())
}
