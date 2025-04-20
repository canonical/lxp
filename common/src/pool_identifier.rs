use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolIdentifier {
    uuid: String,
}

impl PoolIdentifier {
    pub fn new() -> Self {
        PoolIdentifier {
            uuid: Uuid::new_v4().to_string(),
        }
    }

    pub fn from(uuid: String) -> Self {
        PoolIdentifier { uuid }
    }
}

impl ToString for PoolIdentifier {
    fn to_string(&self) -> String {
        self.uuid.clone()
    }
}
