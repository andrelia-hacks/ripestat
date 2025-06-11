use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountryAsnsRequest {
    pub resource: String,
    pub query_time: Option<String>,
    pub lod: Option<i64>,
}
// TODO: lod enum: 0,1

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryAsnsResponse {
    pub countries: Vec<Country>,
    pub resource: Vec<String>,
    pub query_time: String,
    pub lod: Vec<String>,
    pub cache: Option<bool>,
    pub latest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub stats: Stats,
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub registered: i64,
    pub routed: i64,
}
