use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingStatusRequest {
    pub resource: String,
    pub timestamp: Option<String>,
    pub min_peers_seeing: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutingStatusResponse {
    pub first_seen: FirstSeen,
    pub last_seen: LastSeen,
    pub visibility: IpVisibility,

    // Only for resource type AS
    pub announced_space: AnnouncedSpace,
    pub observed_neighbours: i64,

    // Only for resource type prefix
    pub origins: Vec<Origin>,
    pub less_specifics: Vec<SpecificOrigin>,
    pub more_specifics: Vec<SpecificOrigin>,

    pub resource: String,
    pub query_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Origin {
    pub origin: String,
    pub route_objects: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecificOrigin {
    pub prefix: String,
    pub origin: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstSeen {
    pub prefix: String,
    pub origin: String,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastSeen {
    pub prefix: String,
    pub origin: String,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpVisibility {
    pub v4: V4RisPeers,
    pub v6: V6RisPeers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnouncedSpace {
    pub v4: V4RisPeers,
    pub v6: V6RisPeers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V4RisPeers {
    pub ris_peers_seeing: i64,
    pub total_ris_peers: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V6RisPeers {
    pub ris_peers_seeing: i64,
    pub total_ris_peers: i64,
}