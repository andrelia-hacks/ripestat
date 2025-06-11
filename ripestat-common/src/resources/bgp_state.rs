use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BgpStateRequest {
    pub resource: String,
    pub timestamp: Option<String>,
    pub rrcs: Option<String>,
    pub unix_timestamps: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BgpStateResponse {
    pub resource: String,
    pub time: String,
    pub timestamp: String,
    pub bgp_state: BgpState,
    pub nr_routes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BgpState {
    pub target_prefix: String,
    pub path: Vec<String>,
    pub community: Vec<String>,
    pub source_id: String,
}