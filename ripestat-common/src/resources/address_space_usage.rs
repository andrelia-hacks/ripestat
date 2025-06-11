use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressSpaceUsageRequest {
    pub resource: String,
    pub all_level_more_specifics: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressAssignment {
    pub address_range: String,
    pub asn_name: String,
    pub status: String,
    pub parent_allocation: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressAllocation {
    pub allocation: String,
    pub asn_name: String,
    pub status: String,
    pub assignments: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressIpStat {
    pub status: String,
    pub ips: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressSpaceUsageResponse {
    pub query_time: String,
    pub resource: String,
    pub assignments: Vec<AddressAssignment>,
    pub allocations: Vec<AddressAllocation>,
    pub ip_stats: Vec<AddressIpStat>,
}