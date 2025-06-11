use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IanaRegistryInfoRequest {
    pub resource: String,
    pub best_match_only: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IanaRegistryInfoResponse {
    pub resources: Vec<Resource>,
    pub load_time: String,
    pub returned: i64,
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Resource {
    pub resource: String,
    pub type_properties: Vec<String>,
    pub description: String,
    pub details: Details,
    pub source: String,
    pub source_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Details {
    #[serde(rename = "Designation")]
    pub designation: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "WHOIS")]
    pub whois: String,
    #[serde(rename = "RDAP")]
    pub rdap: String,
    #[serde(rename = "Status [1]")]
    pub status_1: String,
    #[serde(rename = "Note")]
    pub note: String,
}
