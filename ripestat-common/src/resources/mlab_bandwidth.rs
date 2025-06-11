use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MlabBandwidthRequest {
    pub resource: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MlabBandwidthResponse {
    pub field: String,
}

// TODO: maintenance endpoint
// https://stat.ripe.net/docs/02.data-api/mlab-bandwidth.html