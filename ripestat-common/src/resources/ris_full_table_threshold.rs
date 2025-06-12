use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RisFullTableThresholdRequest {
    pub query_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisFullTableThresholdResponse {
    pub result_time: String,
    pub v4: i64,
    pub v6: i64,
    pub latest_time: String,
    pub earliest_time: String,
    pub parameters: RisFullTableThresholdParameters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisFullTableThresholdParameters {
    pub query_time: String,
    pub cache: Option<bool>,
}
