use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountryResourceStatsRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
    pub resolution: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryResourceStatsResponse {
    pub query_starttime: String,
    pub query_endtime: String,
    pub stats: Vec<Stat>,
    pub resource: String,
    pub resolution: String,
    pub latest_time: String,
    pub earliest_time: String,
    pub hd_latest_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub timeline: Vec<Timeline>,
    pub v4_prefixes_ris: i64,
    pub v6_prefixes_ris: i64,
    pub asns_ris: i64,
    pub v4_prefixes_stats: i64,
    pub v6_prefixes_stats: i64,
    pub asns_stats: i64,
    pub stats_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timeline {
    pub starttime: String,
    pub endtime: String,
}
