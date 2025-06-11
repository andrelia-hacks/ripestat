use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpkiValidationStatusRequest {
    pub resource: String,
    pub prefix: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpkiValidationStatusResponse {
    pub validating_roas: Vec<ValidatingRoa>,
    pub status: String,
    pub validator: String,
    pub resource: String,
    pub prefix: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatingRoa {
    pub origin: String,
    pub prefix: String,
    pub max_length: i64,
    pub validity: String,
}
