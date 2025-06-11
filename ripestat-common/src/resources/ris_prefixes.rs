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
    pub counts: Counts,
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
pub struct Counts {
    pub v4: V4,
    pub v6: V6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V4 {
    pub originating: i64,
    pub transiting: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V6 {
    pub originating: i64,
    pub transiting: i64,
}
