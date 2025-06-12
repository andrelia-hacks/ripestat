use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RisPrefixesRequest {
    pub resource: String,
    pub query_time: Option<String>,
    pub list_prefixes: Option<bool>,
    pub types: Option<String>,
    pub af: Option<String>,
    pub noise: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisPrefixesResponse {
    pub counts: PrefixCounts,
    pub resource: String,
    pub query_time: String,
    pub list_prefixes: bool,
    pub af: Vec<String>,
    pub types: Vec<String>,
    pub noise: Vec<String>,
    pub cache: Option<bool>,
    pub latest_time: String,
    pub earliest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefixCounts {
    pub v4: V4Prefixes,
    pub v6: V6Prefixes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V4Prefixes {
    pub originating: i64,
    pub transiting: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V6Prefixes {
    pub originating: i64,
    pub transiting: i64,
}
