use anyhow::Result;
use lxp_common::pool_definition::PoolDefinition;
use lxp_daemon_connector::connector::LinuxPoolConnector;

pub fn run_command_list_pools(lxp: &mut LinuxPoolConnector) -> Result<()> {
    let pools: Vec<PoolDefinition> = lxp.list_pools()?;
    for pool in pools {
        println!("{}[{}:{}] ({})", pool.name, pool.live_count, pool.pool_size, pool.series);
    }
    Ok(())
}
