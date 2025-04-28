use lxp_common::{lxp_status::LxpStatus, machine_handle::MachineHandle, pool_definition::PoolDefinition};
use serde::{Deserialize, Serialize};

use crate::serve_target::ServeTarget;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Initiate,
    Begin(ServeTarget),

    DefinePool(PoolDefinition),
    ListPools,
    ListPoolsResponse(Vec<PoolDefinition>),
    GetPool(String),
    GetPoolResponse(PoolDefinition),

    GrabMachine(String),
    GrabMachineResponse(MachineHandle),
    ExecuteCommand(MachineHandle, String),
    ExecuteCommandResponse(String),
    ReleaseMachine(MachineHandle),

    Status,
    StatusResponse(LxpStatus),

    Error(String),

    End,
}
