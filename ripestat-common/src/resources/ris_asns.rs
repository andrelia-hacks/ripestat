use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RisAsnsRequest {
    pub query_time: String,
    pub list_asns: Option<bool>,
    pub asn_types: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisAsnsResponse {
    pub asns: Vec<i64>,
    pub counts: Counts,
    pub query_time: String,
    pub list_asns: bool,
    pub cache: Option<bool>,
    pub latest_time: String,
    pub earliest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Counts {
    pub total: i64,
}
