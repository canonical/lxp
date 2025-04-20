use anyhow::Result;
use clap::Parser;
use lxp_common::pool_definition::PoolDefinition;
use lxp_daemon_connector::connector::LinuxPoolConnector;

#[derive(Debug, Parser)]
pub struct GetPoolArgs {
    #[arg(value_name = "NAME")]
    pool_name: String,
}

pub fn run_command_get_pool(lxp: &mut LinuxPoolConnector, args: GetPoolArgs) -> Result<()> {
    let pool: PoolDefinition = lxp.get_pool(args.pool_name)?;
    println!("{}[{}:{}] ({})", pool.name, pool.live_count, pool.pool_size, pool.series);
    Ok(())
}
