use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountryResourceListRequest {
    pub resource: String,
    pub time: Option<String>,
    pub v4_format: Option<String>,
}
// TODO: v4_format enum: "" or "prefix"

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct CountryResourceListResponse {
    pub query_time: String,
    pub resources: Resources,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Resources {
    pub asn: Vec<String>,
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
}
