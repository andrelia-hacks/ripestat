use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationHistoryRequest {
    pub resource: String,
    pub starttime: Option<bool>,
    pub endtime: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationTimeline {
    pub starttime: String,
    pub endtime: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Allocation {
    pub resource: String,
    pub status: String,
    pub timelines: Vec<AllocationTimeline>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationHistoryResults {
    #[serde(rename = "IANA")]
    pub iana: Option<Vec<Allocation>>,

    #[serde(rename = "AFRINIC")]
    pub afrinic: Option<Vec<Allocation>>,

    #[serde(rename = "APNIC")]
    pub apnic: Option<Vec<Allocation>>,

    #[serde(rename = "ARIN")]
    pub arin: Option<Vec<Allocation>>,

    #[serde(rename = "LACNIC")]
    pub lacnic: Option<Vec<Allocation>>,

    #[serde(rename = "RIPE NCC")]
    pub ripe_ncc: Option<Vec<Allocation>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationHistoryResponse {
    pub results: AllocationHistoryResults,
    pub resource: String,
    pub query_starttime: String,
    pub query_endtime: String,
}