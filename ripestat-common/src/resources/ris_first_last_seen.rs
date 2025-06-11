use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RisFirstLastSeenRequest {
    pub resource: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RisFirstLastSeenResponse {
    pub field: String,
}

// TODO: maintenance endpoint
// https://stat.ripe.net/docs/02.data-api/ris-first-last-seen.html