use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsChainRequest {
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsChainResponse {
    pub query_time: String,
    pub resource: String,
    pub forward_nodes: HashMap<String,Vec<String>>,
    pub reverse_nodes: HashMap<String,Vec<String>>,
    pub authoritative_nameservers: Vec<String>,
    pub nameservers: Vec<String>,
}
