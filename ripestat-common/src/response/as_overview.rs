use serde::{Deserialize, Serialize};

use super::{
    common::Common,
    iana_block::IanaBlock
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AsOverviewData {
    #[serde(rename = "type")]
    pub api_type: String,

    pub block: IanaBlock,

    pub announced: bool,
    pub holder: String,
    pub resource: String,
    pub query_starttime: String,
    pub query_endtime: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AsOverview {
    #[serde(flatten)]
    pub common: Common,

    pub data: AsOverviewData,
}
