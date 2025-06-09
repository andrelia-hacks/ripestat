use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Common {
    pub messages: Vec<Vec<String>>,
    pub see_also: Vec<Vec<String>>,
    pub version: String,
    pub data_call_name: String,
    pub data_call_status: String,
    pub cached: bool,
    pub query_id: String,
    pub process_time: i64,
    pub server_id: String,
    pub build_version: String,
    pub status: String,
    pub status_code: i64,
    pub time: String,
}