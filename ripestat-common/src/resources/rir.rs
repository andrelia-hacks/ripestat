use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RirRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub lod: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RirResponse {
    pub resource: String,
    pub latest: String,
    pub query_starttime: String,
    pub query_endtime: String,
    pub lod: i64,
    pub rirs: Vec<RirSeen>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RirSeen {
    pub rir: String,
    pub first_time: String,
    pub last_time: String,
}
