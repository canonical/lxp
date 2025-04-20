use anyhow::Result;
use lxp_common::pool_definition::PoolDefinition;
use lxp_daemon_connector::connector::LinuxPoolConnector;

use crate::pretty_formats::pool_definition::pool_definitions_as_table;

pub fn run_command_list_pools(lxp: &mut LinuxPoolConnector) -> Result<()> {
    let pools: Vec<PoolDefinition> = lxp.list_pools()?;
    println!("{}", pool_definitions_as_table(pools));
    Ok(())
}
