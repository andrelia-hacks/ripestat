use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RirStatsCountryRequest {
    pub resource: String,
    pub query_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RirStatsCountryResponse {
    pub located_resources: Vec<RirStatsCountryLocatedResource>,
    pub result_time: String,
    pub parameters: RirStatsCountryParameters,
    pub earliest_time: String,
    pub latest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RirStatsCountryLocatedResource {
    pub resource: String,
    pub location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RirStatsCountryParameters {
    pub resource: String,
    pub query_time: String,
    pub cache: Option<bool>,
}
