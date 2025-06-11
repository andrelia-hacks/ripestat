use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaxmindGeoLiteRequest {
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaxmindGeoLiteResponse {
    pub located_resources: Vec<LocatedResource>,
    pub unknown_percentage: UnknownPercentage,
    pub result_time: String,
    pub latest_time: String,
    pub earliest_time: String,
    pub parameters: Parameters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocatedResource {
    pub resource: String,
    pub locations: Vec<Location>,
    pub unknown_percentage: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub country: String,
    pub city: String,
    pub resources: Vec<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub covered_percentage: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnknownPercentage {
    pub v4: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameters {
    pub resource: String,
    pub resolution: String,
    pub cache: Option<bool>,
}
