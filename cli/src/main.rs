use std::path::PathBuf;

use lxp_daemon_connector::connector::LinuxPoolConnector;
use lxp_common::{machine_identifier::MachineIdentifier, pool_definition::PoolDefinition, pool_identifier::PoolIdentifier};

fn main() -> anyhow::Result<()> {
    let mut lxp: LinuxPoolConnector = LinuxPoolConnector::connect()?;

    let pool_definition_path: PathBuf = PathBuf::from("/home/thinking-dragon/source-wand.lxp.yaml");
    lxp.define_pool(PoolDefinition::from_file(pool_definition_path)?)?;

    let pools: Vec<PoolDefinition> = lxp.list_pools()?;
    println!("Pools:");
    for pool in pools {
        println!("\t{}[{}:{}] ({})", pool.name, pool.live_count, pool.pool_size, pool.series);
    }

    let pool: PoolDefinition = lxp.get_pool("source-wand".to_string())?;
    println!("Pool \"source-wand\":");
    println!("\t{}[{}:{}] ({})", pool.name, pool.live_count, pool.pool_size, pool.series);

    let machine_id: MachineIdentifier = lxp.grab_machine(PoolIdentifier::new())?;
    println!("{}", machine_id.to_string());

    Ok(())
}
