use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsPathLengthRequest {
    pub resource: String,
    pub sort_by: Option<String>,
}
// TODO: sort_by enum
// "number", "count", "location", "geo"

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsPathLengthResponse {
    pub stats: Vec<Stat>,
    pub resource: String,
    pub query_time: String,
    pub sort_by: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stat {
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
