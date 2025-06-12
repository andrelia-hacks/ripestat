use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RisPeersRequest {
    pub query_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisPeersResponse {
    pub peers: HashMap<String, Vec<RisPeersPeer>>,
    pub latest_time: String,
    pub earliest_time: String,
    pub parameters: RisPeersParameters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisPeersPeer {
    pub asn: i64,
    pub ip: String,
    pub v4_prefix_count: i64,
    pub v6_prefix_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisPeersParameters {
    pub query_time: String,
    pub cache: Option<bool>,
}
