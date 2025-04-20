use lxp_common::{machine_identifier::MachineIdentifier, pool_definition::PoolDefinition, pool_identifier::PoolIdentifier};
use serde::{Deserialize, Serialize};

use crate::serve_target::ServeTarget;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Initiate,
    Begin(ServeTarget),

    DefinePool(PoolDefinition),

    GrabMachine(PoolIdentifier),
    GrabMachineResponse(MachineIdentifier),

    End,
}
