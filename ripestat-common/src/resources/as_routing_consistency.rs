use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsRoutingConsistencyRequest {
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsRoutingConsistencyResponse {
    pub prefixes: Vec<Prefix>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
    pub authority: String,
    pub resource: String,
    pub parameters: Parameters,
    pub query_starttime: String,
    pub query_endtime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prefix {
    pub in_bgp: bool,
    pub in_whois: bool,
    pub irr_sources: Vec<String>,
    pub prefix: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub in_bgp: bool,
    pub in_whois: bool,
    pub peer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Export {
    pub in_bgp: bool,
    pub in_whois: bool,
    pub peer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameters {
    pub resource: String,
    pub query_time: String,
    pub cache: String,
}
