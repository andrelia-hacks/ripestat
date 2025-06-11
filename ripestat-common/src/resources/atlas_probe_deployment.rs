use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AtlasProbeDeploymentRequest {
    pub resource: String,
    pub starttime: Option<String>,
    pub endtime: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtlasProbeDeploymentResponse {
    pub deployments: Vec<ResourceDeployment>,
    pub resource: Vec<String>,
    pub starttime: String,
    pub endtime: String,
    pub query_date: String,
    pub merge: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDeployment {
    pub resource: String,
    pub deployment: Vec<Deployment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    pub date: String,
    pub statuses: Statuses,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statuses {
    pub neverseen: i64,
    pub connected: i64,
    pub disconnected: i64,
    pub abandoned: i64,
}