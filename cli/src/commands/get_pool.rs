use anyhow::Result;
use clap::Parser;
use colorize::AnsiColor;
use lxp_common::pool_definition::PoolDefinition;
use lxp_daemon_connector::connector::LinuxPoolConnector;

#[derive(Debug, Parser)]
pub struct GetPoolArgs {
    #[arg(value_name = "POOL")]
    pool_name: String,
}

pub fn run_command_get_pool(lxp: &mut LinuxPoolConnector, args: GetPoolArgs) -> Result<()> {
    let pool: PoolDefinition = lxp.get_pool(args.pool_name)?;

    println!("\n{}\n", "Pool information".underlined());
    println!("{}: {}", "      Name".bold(), pool.name);
    println!("{}: {}", "   Base OS".bold(), pool.base);
    println!("{}: {} (number of machines running at any point in time)", "Live count".bold(), pool.live_count);
    println!("{}: {} (number of machines in pool)\n", " Pool size".bold(), pool.pool_size);

    Ok(())
}
