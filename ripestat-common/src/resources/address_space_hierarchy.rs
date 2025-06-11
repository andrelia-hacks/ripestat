use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressSpaceHierarchyRequest {
    pub resource: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressSpaceHierarchyParameters {
    pub resource: String
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressSpaceObject {
    pub inetnum: String,
    pub netname: String,
    pub descr: String,
    pub org: String,
    pub remarks: String,
    pub country: String,

    #[serde(rename = "admin-c")]
    pub admin_c: String,

    #[serde(rename = "tech-c")]
    pub tech_c: String,

    pub status: String,

    #[serde(rename = "mnt-by")]
    pub mnt_by: String,

    #[serde(rename = "mnt-routes")]
    pub mnt_routes: String,

    pub created: String,

    #[serde(rename = "last-modified")]
    pub last_modified: String,

    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressSpaceHierarchyResponse {
    pub rir: String,
    pub resource: String,
    pub exact: Vec<AddressSpaceObject>,
    pub less_specific: Vec<AddressSpaceObject>,
    pub more_specific: Vec<AddressSpaceObject>,
    pub query_time: String,
    pub parameters: AddressSpaceHierarchyParameters,
}