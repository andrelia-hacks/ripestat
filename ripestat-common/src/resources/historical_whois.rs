use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalWhoisRequest {
    pub resource: String,
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalWhoisResponse {
    pub terms_and_conditions: String,
    pub num_versions: i64,
    pub resource: String,
    #[serde(rename = "type")]
    pub whois_type: String,
    pub database: String,
    pub access: String,
    pub suggestions: Vec<WhoisSuggestion>,
    pub version: String,
    pub cache: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhoisSuggestion {
    #[serde(rename = "type")]
    pub suggestion_type: String,
    pub key: String,
    pub attributes: Vec<Attribute>,
    pub from_time: Option<String>,
    pub version: String,
    pub latest: bool,
    pub deleted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub attribute: String,
    pub value: String,
}
