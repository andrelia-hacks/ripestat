use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeternetBandwidthMeasurementsRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeternetBandwidthMeasurementsResponse {
    pub measurements: Vec<Measurement>,
    pub statistics: MeternetStatistics,
    pub resource: String,
    pub starttime: String,
    pub endtime: String,
    pub cache: Option<bool>,
    pub latest_time: String,
    pub earliest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    pub prefix: String,
    pub date: String,
    pub down: i64,
    pub up: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeternetStatistics {
    pub measurements: i64,
}
