use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpkiHistoryRequest {
    pub resource: String,
    pub family: Option<i64>,
    pub resolution: Option<String>,
    pub include: Option<String>,
    pub delegated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpkiHistoryResponse {
    pub timeseries: Vec<Series>,
    pub resource: String,
    pub delegated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Series {
    pub cc: String,
    pub time: String,
    pub delegated: Delegated,
    pub warnings: Vec<Warning>,
    pub family: i64,
    pub rpki: Rpki,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Delegated {
    pub prefixes: Prefixes,
    pub space: Space,
    pub samples: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prefixes {
    pub count: f64,
    pub covered_by_rpki: CoveredByRpki,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Space {
    pub count: f64,
    pub covered_by_rpki: CoveredByRpki,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoveredByRpki {
    pub count: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Warning {
    pub dimension: String,
    pub reason: String,
    pub source: String,
    pub comment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rpki {
    pub vrp_count: i64,
}
