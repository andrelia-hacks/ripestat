use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, strum_macros::Display)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    Number,
    Count,
    Location,
    Geo,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsPathLengthRequest {
    pub resource: String,
    pub sort_by: Option<SortBy>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsPathLengthResponse {
    pub stats: Vec<AsPathLengthStat>,
    pub resource: String,
    pub query_time: String,
    pub sort_by: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsPathLengthStat {
    pub number: i64,
    pub count: i64,
    pub location: String,
    pub stripped: Stripped,
    pub unstripped: Unstripped,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stripped {
    pub sum: i64,
    pub min: i64,
    pub max: i64,
    pub avg: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unstripped {
    pub sum: i64,
    pub min: i64,
    pub max: i64,
    pub avg: f64,
}
