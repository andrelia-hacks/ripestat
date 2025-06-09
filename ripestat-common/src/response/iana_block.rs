use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IanaBlock {
    pub resource: String,
    pub desc: String,
    pub name: String,
}