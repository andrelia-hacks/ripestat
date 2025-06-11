use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchcompleteRequest {
    pub resource: String,
    pub limit: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchcompleteResponse {
    pub categories: Vec<Category>,
    pub query_term: String,
    pub limit: i64,
    pub query_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub category: String,
    pub suggestions: Vec<Suggestion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suggestion {
    pub label: String,
    pub value: String,
    pub description: String,
    pub link: Option<String>,
}
