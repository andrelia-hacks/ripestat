use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhoisRequest {
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhoisResponse {
    pub records: Vec<Vec<Record>>,
    pub irr_records: Vec<Vec<IrrRecord>>,
    pub authorities: Vec<String>,
    pub resource: String,
    pub query_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub key: String,
    pub value: String,
    pub details_link: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrrRecord {
    pub key: String,
    pub value: String,
    pub details_link: Option<String>,
}
