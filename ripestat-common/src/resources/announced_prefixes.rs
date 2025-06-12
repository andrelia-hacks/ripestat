use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnouncedPrefixesRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub min_peers_seeing: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefixTimeline {
    pub starttime: String,
    pub endtime: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnouncedPrefix {
    pub prefix: String,
    pub timelines: Vec<PrefixTimeline>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnouncedPrefixesResponse {
    pub prefixes: Vec<AnnouncedPrefix>,

    pub resource: String,

    pub query_starttime: String,
    pub query_endtime: String,

    pub latest_time: String,
    pub earliest_time: String,
}