use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefixOverviewRequest {
    pub resource: String,
    pub min_peers_seeing: Option<i64>,
    pub max_related: Option<i64>,
    pub query_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefixOverviewResponse {
    pub is_less_specific: bool,
    pub announced: bool,
    pub asns: Vec<Asn>,
    pub related_prefixes: Vec<Value>,
    pub resource: String,
    #[serde(rename = "type")]
    pub prefix_overview_type: String,
    pub block: Block,
    pub actual_num_related: i64,
    pub query_time: String,
    pub num_filtered_out: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asn {
    pub asn: i64,
    pub holder: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub resource: String,
    pub desc: String,
    pub name: String,
}
