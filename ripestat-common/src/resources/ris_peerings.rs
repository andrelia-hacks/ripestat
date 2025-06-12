use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RisPeeringsRequest {
    pub resource: String,
    pub query_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisPeeringsResponse {
    pub peerings: Vec<Peering>,
    pub resource: String,
    pub query_starttime: String,
    pub query_endtime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Peering {
    pub probe: RisAtlasProbe,
    pub peers: Vec<RisPeeringsPeer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisAtlasProbe {
    pub city: String,
    pub country: String,
    pub longitude: f64,
    pub latitude: f64,
    pub name: String,
    pub ipv4_peer_count: i64,
    pub ipv6_peer_count: i64,
    pub ixp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RisPeeringsPeer {
    pub asn: i64,
    pub ip: String,
    pub ip_version: String,
    pub table_version: String,
    pub prefix_count: i64,
    pub routes: Vec<PeerRoute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeerRoute {
    pub as_path: Vec<i64>,
}
