use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RirStatsCountryRequest {
    pub resource: String,
    pub query_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RirStatsCountryResponse {
    pub located_resources: Vec<LocatedResource>,
    pub result_time: String,
    pub parameters: Parameters,
    pub earliest_time: String,
    pub latest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocatedResource {
    pub resource: String,
    pub location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameters {
    pub resource: String,
    pub query_time: String,
    pub cache: Option<bool>,
}
