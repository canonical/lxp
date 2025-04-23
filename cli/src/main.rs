use anyhow::Result;
use clap::{
    command,
    Parser,
    Subcommand
};
use commands::{
    define_pool::{run_command_define_pool, DefinePoolArgs}, get_pool::{run_command_get_pool, GetPoolArgs}, grab_machine::{run_command_grab_machine, GrabMachineArgs}, list_pools::run_command_list_pools
};
use lxp_daemon_connector::connector::LinuxPoolConnector;

mod commands;
mod pretty_formats;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Update a pool definition")]
    DefinePool(DefinePoolArgs),

    #[command(about = "List all pools")]
    Pools,

    #[command(about = "Get specific pool")]
    Pool(GetPoolArgs),

    #[command(about = "Grab machine from pool")]
    GrabMachine(GrabMachineArgs),
}

fn main() -> Result<()> {
    let mut lxp: LinuxPoolConnector = LinuxPoolConnector::connect()?;
    
    match Cli::parse().command {
        Command::DefinePool(args) => run_command_define_pool(&mut lxp, args),
        Command::Pools => run_command_list_pools(&mut lxp),
        Command::Pool(args) => run_command_get_pool(&mut lxp, args),
        Command::GrabMachine(args) => run_command_grab_machine(&mut lxp, args),
    }
}
