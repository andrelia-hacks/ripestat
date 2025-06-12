use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RirGeoRequest {
    pub resource: String,
    pub query_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RirGeoResponse {
    pub located_resources: Vec<RirGeoLocatedResource>,
    pub result_time: String,
    pub parameters: RirGeoParameters,
    pub earliest_time: String,
    pub latest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RirGeoLocatedResource {
    pub resource: String,
    pub location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RirGeoParameters {
    pub resource: String,
    pub query_time: String,
    pub cache: Option<bool>,
}
