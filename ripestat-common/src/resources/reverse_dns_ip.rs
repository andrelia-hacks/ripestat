use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReverseDnsIpRequest {
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReverseDnsIpResponse {
    pub query_time: String,
    pub resource: String,
    pub result: Vec<String>,
    pub error: String,
}
