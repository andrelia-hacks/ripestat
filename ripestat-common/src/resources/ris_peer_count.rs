use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RisPeerCountRequest {
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub v4_full_prefix_threshold: Option<i64>,
    pub v6_full_prefix_threshold: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisPeerCountResponse {
    pub starttime: String,
    pub endtime: String,
    pub peer_count: PeerCount,
    pub v4_full_prefix_threshold: i64,
    pub v6_full_prefix_threshold: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeerCount {
    pub v4: V4,
    pub v6: V6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V4 {
    pub total: Vec<Total>,
    pub full_feed: Vec<FullFeed>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V6 {
    pub total: Vec<Total>,
    pub full_feed: Vec<FullFeed>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Total {
    pub timestamp: String,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullFeed {
    pub timestamp: String,
    pub count: i64,
}