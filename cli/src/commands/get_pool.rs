use anyhow::Result;
use clap::Parser;
use colorize::AnsiColor;
use lxp_common::pool_definition::PoolDefinition;
use lxp_daemon_connector::connector::LinuxPoolConnector;

#[derive(Debug, Parser)]
pub struct GetPoolArgs {
    #[arg(value_name = "NAME")]
    pool_name: String,
}

pub fn run_command_get_pool(lxp: &mut LinuxPoolConnector, args: GetPoolArgs) -> Result<()> {
    let pool: PoolDefinition = lxp.get_pool(args.pool_name)?;

    let title: String = "      Pool".bold();
    println!("{}: {}", title, pool.name);
    let title: String = "   Base OS".bold();
    println!("{}: {}", title, pool.base);
    let title: String = "Live count".bold();
    println!("{}: {} (number of machines running at any point in time)", title, pool.live_count);
    let title: String = " Pool size".bold();
    println!("{}: {} (number of machines in pool)", title, pool.pool_size);

    Ok(())
}
