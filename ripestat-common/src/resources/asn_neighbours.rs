use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsnNeighboursRequest {
    pub resource: String,
    pub query_time: Option<String>,
    pub lod: Option<i64>,
}
// TODO: lod enum
// 0, 1

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsnNeighboursResponse {
    pub resource: String,
    pub query_starttime: String,
    pub query_endtime: String,
    pub latest_time: String,
    pub earliest_time: String,
    pub neighbour_counts: NeighbourCounts,
    pub neighbours: Vec<Neighbour>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeighbourCounts {
    pub left: i64,
    pub right: i64,
    pub unique: i64,
    pub uncertain: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Neighbour {
    pub asn: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub power: i64,
    pub v4_peers: i64,
    pub v6_peers: i64,
}
