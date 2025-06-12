use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeedcheckerBandwidthMeasurementsRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeedcheckerBandwidthMeasurementsResponse {
    pub measurements: Vec<Value>,
    pub statistics: SpeedcheckerStatistics,
    pub resource: String,
    pub starttime: String,
    pub endtime: String,
    pub cache: Option<bool>,
    pub earliest_time: String,
    pub latest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeedcheckerStatistics {
    pub measurements: i64,
}
