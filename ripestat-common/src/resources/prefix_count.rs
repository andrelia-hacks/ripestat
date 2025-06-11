use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefixCountRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub min_peers_seeing: Option<i64>,
    pub resolution: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefixCountResponse {
    pub ipv4: Vec<Ipv4>,
    pub ipv6: Vec<Ipv6>,
    pub resource: String,
    pub query_starttime: String,
    pub query_endtime: String,
    pub resolution: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ipv4 {
    pub prefixes: i64,
    pub timestamp: String,
    #[serde(rename = "address-space")]
    pub address_space: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ipv6 {
    pub prefixes: i64,
    pub timestamp: String,
    #[serde(rename = "address-space")]
    pub address_space: i64,
}

