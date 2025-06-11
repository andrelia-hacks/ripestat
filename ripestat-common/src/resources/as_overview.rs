use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsOverviewRequest {
    pub resource: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IanaBlock {
    pub resource: String,
    pub desc: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsOverviewResponse {
    #[serde(rename = "type")]
    pub data_type: String,

    pub block: IanaBlock,

    pub announced: bool,
    pub holder: String,
    pub resource: String,
    pub query_starttime: String,
    pub query_endtime: String,
}