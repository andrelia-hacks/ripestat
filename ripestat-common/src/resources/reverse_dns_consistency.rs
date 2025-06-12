use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReverseDnsConsistencyRequest {
    pub resource: String,
    pub ipv4: Option<bool>,
    pub ipv6: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReverseDnsConsistencyResponse {
    pub prefixes: Prefixes,
    pub query_time: String,
    pub resource: String,
    pub source: String,
    pub ipv4: bool,
    pub ipv6: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prefixes {
    pub ipv4: HashMap<String,ReverseDnsPrefix>,
    pub ipv6: HashMap<String,ReverseDnsPrefix>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReverseDnsPrefix {
    pub complete: bool,
    pub domains: Vec<Domain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    pub domain: String,
    pub prefix: String,
    pub found: bool,
}