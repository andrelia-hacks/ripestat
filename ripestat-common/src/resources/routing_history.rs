use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingHistoryRequest {
    pub resource: String,
    pub max_rows: Option<i64>,
    pub include_first_hop: Option<bool>,
    pub normalise_visibility: Option<bool>,
    pub min_peers: Option<i64>,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutingHistoryResponse {
    pub by_origin: Vec<ByOrigin>,
    pub resource: String,
    pub query_starttime: String,
    pub query_endtime: String,
    pub time_granularity: i64,
    pub latest_max_ff_peers: LatestMaxFfPeers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByOrigin {
    pub origin: String,
    pub prefixes: Vec<Prefix>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prefix {
    pub prefix: String,
    pub timelines: Vec<RoutingHistoryTimeline>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutingHistoryTimeline {
    pub starttime: String,
    pub endtime: String,
    pub full_peers_seeing: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatestMaxFfPeers {
    pub v4: i64,
    pub v6: i64,
}
