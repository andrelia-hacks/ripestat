use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BgpUpdateActivityRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub max_samples: Option<i64>,
    pub min_sampling_period: Option<i64>,
    pub num_hours: Option<i64>,
    pub hide_empty_samples: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct BgpUpdateActivityResponse {
    pub max_samples: i64,
    pub query_endtime: String,
    pub query_starttime: String,
    pub related_prefixes: Vec<Value>,
    pub resource: String,
    pub resource_type: String,
    pub sampling_period: f64,
    pub sampling_period_human: String,
    pub updates: Vec<BgpUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct BgpUpdate {
    pub announcements: i64,
    pub withdrawals: i64,
    pub starttime: String,
}

