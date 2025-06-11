use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BgplayRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub rrcs: Option<String>,
    pub unix_timestamps: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BgplayResponse {
    pub field: String,
}

// TODO: maintenance endpoint
// https://stat.ripe.net/docs/02.data-api/bgplay.html