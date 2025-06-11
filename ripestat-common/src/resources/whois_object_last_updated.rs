use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhoisObjectLastUpdatedRequest {
    pub object: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub source: String,
    pub timestamp: Option<String>,
    pub compare_with_live: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhoisObjectLastUpdatedResponse {
    pub field: String,
}

// TODO: maint
// https://stat.ripe.net/docs/02.data-api/whois-object-last-updated.html