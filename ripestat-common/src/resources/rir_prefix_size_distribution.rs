use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RirPrefixSizeDistributionRequest {
    pub resource: String,
    pub query_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RirPrefixSizeDistributionResponse {
    pub resource: String,
    pub query_time: String,
    pub rirs: Vec<Rir>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rir {
    pub rir: String,
    pub distribution: Vec<Distribution>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Distribution {
    pub prefix_size: i64,
    pub count: i64,
}
