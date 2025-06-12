use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AtlasProbesRequest {
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtlasProbesResponse {
    pub probes: Vec<Probe>,
    pub stats: AtlasProbesStats,
    pub resource: String,
    pub cache: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Probe {
    pub prefix_v4: Option<String>,
    pub status: i64,
    pub status_name: String,
    pub prefix_v6: Option<String>,
    pub is_anchor: bool,
    pub last_connected: Option<i64>,
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub address_v6: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub id: i64,
    pub address_v4: Option<String>,
    pub country_code: String,
    pub is_public: bool,
    pub asn_v4: Option<i64>,
    pub asn_v6: Option<i64>,
    pub status_since: Option<i64>,
    pub first_connected: Option<i64>,
    pub total_uptime: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtlasProbesStats {
    pub total: i64,
}
