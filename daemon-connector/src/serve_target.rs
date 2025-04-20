use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServeTarget {
    Root,
    Client(String),
}

impl ServeTarget {
    pub fn is_root(&self) -> bool {
        self == &ServeTarget::Root
    }
}
