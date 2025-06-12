use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsnNeighboursHistoryRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub max_rows: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsnNeighboursHistoryResponse {
    pub resource: String,
    pub query_starttime: String,
    pub query_endtime: String,
    pub neighbours: Vec<AsnHistoryNeighbour>,
    pub latest_time: String,
    pub earliest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsnHistoryNeighbour {
    pub neighbour: String,
    pub timelines: Vec<AsnNeighboursHistoryTimeline>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsnNeighboursHistoryTimeline {
    pub starttime: String,
    pub endtime: String,
}