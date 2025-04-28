use anyhow::Result;

use lxp_common::lxp_status::LxpStatus;
use lxp_daemon_connector::connector::LinuxPoolConnector;

use crate::pretty_formats::status::{display_pools, display_units};

pub fn run_command_status(lxp: &mut LinuxPoolConnector) -> Result<()> {
    let status: LxpStatus = lxp.status()?;

    display_pools(status.pools);
    print!("\n");
    display_units(status.units);

    Ok(())
}
