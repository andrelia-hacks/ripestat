use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefixSizeDistributionRequest {
    pub resource: String,
    pub timestamp: Option<String>,
    pub min_peers_seeing: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefixSizeDistributionResponse {
    pub resource: String,
    pub query_time: String,
    pub ipv4: Vec<Ipv4Count>,
    pub ipv6: Vec<Ipv6Count>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ipv4Count {
    pub size: i64,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ipv6Count {
    pub size: i64,
    pub count: i64,
}
