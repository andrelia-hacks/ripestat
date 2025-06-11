use thiserror::Error;

#[doc(inline)]
pub use abuse_contact_finder::*;
#[doc(inline)]
pub use address_space_hierarchy::*;
#[doc(inline)]
pub use address_space_usage::*;
#[doc(inline)]
pub use allocation_history::*;
#[doc(inline)]
pub use announced_prefixes::*;
#[doc(inline)]
pub use as_overview::*;
#[doc(inline)]
pub use as_path_length::*;
#[doc(inline)]
pub use as_routing_consistency::*;
#[doc(inline)]
pub use asn_neighbours::*;
#[doc(inline)]
pub use asn_neighbours_history::*;
#[doc(inline)]
pub use atlas_probe_deployment::*;
#[doc(inline)]
pub use atlas_probes::*;
#[doc(inline)]
pub use atlas_targets::*;
#[doc(inline)]
pub use bgp_state::*;
#[doc(inline)]
pub use bgp_update_activity::*;
#[doc(inline)]
pub use bgp_updates::*;
#[doc(inline)]
pub use bgplay::*;
#[doc(inline)]
pub use country_asns::*;
#[doc(inline)]
pub use country_resource_list::*;
#[doc(inline)]
pub use country_resource_stats::*;
#[doc(inline)]
pub use dns_chain::*;
#[doc(inline)]
pub use historical_whois::*;
#[doc(inline)]
pub use iana_registry_info::*;
#[doc(inline)]
pub use looking_glass::*;
#[doc(inline)]
pub use maxmind_geo_lite::*;
#[doc(inline)]
pub use maxmind_geo_lite_announced_by_as::*;
#[doc(inline)]
pub use meternet_bandwidth_measurements::*;
#[doc(inline)]
pub use mlab_activity_count::*;
#[doc(inline)]
pub use mlab_bandwidth::*;
#[doc(inline)]
pub use mlab_clients::*;
#[doc(inline)]
pub use network_info::*;
#[doc(inline)]
pub use prefix_count::*;
#[doc(inline)]
pub use prefix_overview::*;
#[doc(inline)]
pub use prefix_routing_consistency::*;
#[doc(inline)]
pub use prefix_size_distribution::*;
#[doc(inline)]
pub use related_prefixes::*;
#[doc(inline)]
pub use reverse_dns::*;
#[doc(inline)]
pub use reverse_dns_consistency::*;
#[doc(inline)]
pub use reverse_dns_ip::*;
#[doc(inline)]
pub use rir::*;
#[doc(inline)]
pub use rir_geo::*;
#[doc(inline)]
pub use rir_prefix_size_distribution::*;
#[doc(inline)]
pub use rir_stats_country::*;
#[doc(inline)]
pub use ris_asns::*;
#[doc(inline)]
pub use ris_first_last_seen::*;
#[doc(inline)]
pub use ris_full_table_threshold::*;
#[doc(inline)]
pub use ris_peer_count::*;
#[doc(inline)]
pub use ris_peerings::*;
#[doc(inline)]
pub use ris_peers::*;
#[doc(inline)]
pub use ris_prefixes::*;
#[doc(inline)]
pub use routing_history::*;
#[doc(inline)]
pub use routing_status::*;
#[doc(inline)]
pub use rpki_history::*;
#[doc(inline)]
pub use rpki_validation_status::*;
#[doc(inline)]
pub use rrc_info::*;
#[doc(inline)]
pub use searchcomplete::*;
#[doc(inline)]
pub use speedchecker_bandwidth_measurements::*;
#[doc(inline)]
pub use visibility::*;
#[doc(inline)]
pub use whois::*;
#[doc(inline)]
pub use whois_object_last_updated::*;

pub(crate) mod abuse_contact_finder;
pub(crate) mod address_space_hierarchy;
pub(crate) mod address_space_usage;
pub(crate) mod allocation_history;
pub(crate) mod announced_prefixes;
pub(crate) mod as_overview;
pub(crate) mod as_path_length;
pub(crate) mod as_routing_consistency;
pub(crate) mod asn_neighbours;
pub(crate) mod asn_neighbours_history;
pub(crate) mod atlas_probe_deployment;
pub(crate) mod atlas_probes;
pub(crate) mod atlas_targets;
pub(crate) mod bgp_state;
pub(crate) mod bgp_update_activity;
pub(crate) mod bgp_updates;
pub(crate) mod bgplay;
pub(crate) mod country_asns;
pub(crate) mod country_resource_list;
pub(crate) mod country_resource_stats;
pub(crate) mod dns_chain;
pub(crate) mod historical_whois;
pub(crate) mod iana_registry_info;
pub(crate) mod looking_glass;
pub(crate) mod maxmind_geo_lite;
pub(crate) mod maxmind_geo_lite_announced_by_as;
pub(crate) mod meternet_bandwidth_measurements;
pub(crate) mod mlab_activity_count;
pub(crate) mod mlab_bandwidth;
pub(crate) mod mlab_clients;
pub(crate) mod network_info;
pub(crate) mod prefix_count;
pub(crate) mod prefix_overview;
pub(crate) mod prefix_routing_consistency;
pub(crate) mod prefix_size_distribution;
pub(crate) mod related_prefixes;
pub(crate) mod reverse_dns;
pub(crate) mod reverse_dns_consistency;
pub(crate) mod reverse_dns_ip;
pub(crate) mod rir;
pub(crate) mod rir_geo;
pub(crate) mod rir_prefix_size_distribution;
pub(crate) mod rir_stats_country;
pub(crate) mod ris_asns;
pub(crate) mod ris_first_last_seen;
pub(crate) mod ris_full_table_threshold;
pub(crate) mod ris_peer_count;
pub(crate) mod ris_peerings;
pub(crate) mod ris_peers;
pub(crate) mod ris_prefixes;
pub(crate) mod routing_history;
pub(crate) mod routing_status;
pub(crate) mod rpki_history;
pub(crate) mod rpki_validation_status;
pub(crate) mod rrc_info;
pub(crate) mod searchcomplete;
pub(crate) mod speedchecker_bandwidth_measurements;
pub(crate) mod visibility;
pub(crate) mod whois;
pub(crate) mod whois_object_last_updated;

#[derive(Debug, Error)]
pub enum RipestatResponseError {
    /// An error has occurred parsing the JSON.
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    /// An error has occurred during the request
    #[error(transparent)]
    Client(#[from] reqwest::Error),
}