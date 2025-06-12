use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisibilityRequest {
    pub resource: String,
    pub query_time: Option<String>,
    pub include: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisibilityResponse {
    pub visibilities: Vec<Visibility>,
    pub resource: String,
    pub related_prefixes: Vec<Value>,
    pub query_time: String,
    pub latest_time: String,
    pub include: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Visibility {
    pub probe: VisibilityProbe,
    pub ipv4_full_table_peers_not_seeing: Vec<Ipv4FullTablePeersNotSeeing>,
    pub ipv6_full_table_peers_not_seeing: Vec<Ipv6FullTablePeersNotSeeing>,
    pub ipv4_full_table_peer_count: i64,
    pub ipv6_full_table_peer_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisibilityProbe {
    pub city: String,
    pub country: String,
    pub longitude: f64,
    pub latitude: f64,
    pub name: String,
    pub ipv4_peer_count: i64,
    pub ipv6_peer_count: i64,
    pub ixp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ipv4FullTablePeersNotSeeing {
    pub asn: i64,
    pub ip: String,
    pub prefix_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ipv6FullTablePeersNotSeeing {
    pub asn: i64,
    pub ip: String,
    pub prefix_count: i64,
}
