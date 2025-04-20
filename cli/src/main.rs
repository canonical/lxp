use lxp_daemon_connector::connector::LinuxPoolConnector;
use lxp_common::{machine_identifier::MachineIdentifier, pool_definition::PoolDefinition, pool_identifier::PoolIdentifier};

fn main() -> anyhow::Result<()> {
    let mut lxp: LinuxPoolConnector = LinuxPoolConnector::connect()?;

    lxp.define_pool(PoolDefinition::new(5, 3))?;

    let machine_id: MachineIdentifier = lxp.grab_machine(PoolIdentifier::new())?;
    println!("{}", machine_id.to_string());

    Ok(())
}
