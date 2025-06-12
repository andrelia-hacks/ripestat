use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedPrefixesRequest {
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedPrefixesResponse {
    pub resource: String,
    pub prefixes: Vec<RelatedPrefix>,
    pub query_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedPrefix {
    pub prefix: String,
    pub origin_asn: i64,
    pub asn_name: String,
    pub relationship: String,
}
