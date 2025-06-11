use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbuseContactFinderRequest {
    pub resource: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbuseContactFinderParameters {
    pub resource: String,
    pub cache: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbuseContactFinderResponse {
    pub abuse_contacts: Vec<String>,
    pub authoritative_rir: String,
    pub latest_time: String,
    pub earliest_time: String,
    pub parameters: AbuseContactFinderParameters,
}