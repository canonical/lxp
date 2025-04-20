use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use lxp_common::pool_definition::PoolDefinition;
use lxp_daemon_connector::connector::LinuxPoolConnector;

#[derive(Debug, Parser)]
pub struct DefinePoolArgs {
    #[arg(value_name = "FILE")]
    pool_definition_file: PathBuf,
}

pub fn run_command_define_pool(lxp: &mut LinuxPoolConnector, args: DefinePoolArgs) -> Result<()> {
    let pool_definition: PoolDefinition = PoolDefinition::from_file(args.pool_definition_file)?;
    lxp.define_pool(pool_definition)?;
    Ok(())
}
