use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MlabActivityCountRequest {
    pub resource: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MlabActivityCountResponse {
    pub field: String,
}

// TODO: maintenance endpoint
// https://stat.ripe.net/docs/02.data-api/mlab-activity-count.html