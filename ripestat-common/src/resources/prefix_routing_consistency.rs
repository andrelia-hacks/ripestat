use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefixRoutingConsistencyRequest {
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefixRoutingConsistencyResponse {
    pub resource: String,
    pub routes: Vec<Route>,
    pub parameters: PrefixRoutingConsistencyParameters,
    pub query_starttime: String,
    pub query_endtime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    pub in_bgp: bool,
    pub in_whois: bool,
    pub prefix: String,
    pub origin: i64,
    pub irr_sources: Vec<String>,
    pub asn_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefixRoutingConsistencyParameters {
    pub resource: String,
    pub data_overload_limit: String,
}
