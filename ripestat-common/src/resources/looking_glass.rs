use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LookingGlassRequest {
    pub resource: String,
    pub look_back_limit: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookingGlassResponse {
    pub rrcs: Vec<Rrc>,
    pub query_time: String,
    pub latest_time: String,
    pub parameters: Parameters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rrc {
    pub rrc: String,
    pub location: String,
    pub peers: Vec<Peer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Peer {
    pub asn_origin: String,
    pub as_path: String,
    pub community: String,
    pub large_community: String,
    pub extended_community: String,
    pub last_updated: String,
    pub prefix: String,
    pub peer: String,
    pub origin: String,
    pub next_hop: String,
    pub latest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameters {
    pub resource: String,
    pub look_back_limit: i64,
    pub cache: Option<bool>,
}
