use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MlabClientsRequest {
    pub resource: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MlabClientsResponse {
    pub field: String,
}

// TODO: maintenance endpoint
// https://stat.ripe.net/docs/02.data-api/mlab-clients.html