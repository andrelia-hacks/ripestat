use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RrcInfoResponse {
    pub rrcs: Vec<Rrc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rrc {
    pub id: i64,
    pub name: String,
    pub geographical_location: String,
    pub topological_location: String,
    pub multihop: bool,
    pub activated_on: String,
    pub deactivated_on: String,
    pub peers: Vec<RrcInfoPeer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RrcInfoPeer {
    pub asn: i64,
    pub ip: String,
    pub v4_prefix_count: i64,
    pub is_full_feed_v4: bool,
    pub v6_prefix_count: i64,
    pub is_full_feed_v6: bool,
}
