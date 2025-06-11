use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BgpUpdatesRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub rrcs: Option<String>,
    pub unix_timestamps: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BgpUpdatesResponse {
    pub resource: String,
    pub query_starttime: String,
    pub query_endtime: String,
    pub updates: Vec<Update>,
    pub nr_updates: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Update {
    pub seq: i64,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub update_type: String,
    pub attrs: Attrs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attrs {
    pub source_id: String,
    pub target_prefix: String,
    pub path: Vec<i64>,
    pub community: Vec<String>,
}
