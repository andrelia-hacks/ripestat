use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AtlasTargetsRequest {
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtlasTargetsResponse {
    pub measurements: Vec<Measurement>,
    pub stats: Stats,
    pub resource: String,
    pub authenticated: bool,
    pub cache: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    pub af: i64,
    pub msm_id: i64,
    pub stop_time: Option<i64>,
    pub start_time: i64,
    pub dst_name: String,
    pub dst_addr: String,
    pub dst_asn: Option<i64>,
    pub status: Status,
    pub type_field: Type,
    pub creation_time: i64,
    pub description: String,
    pub result: String,
    pub size: Option<i64>,
    pub is_public: bool,
    pub participant_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub name: String,
    pub id: i64,
    pub when: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub total: i64,
}
