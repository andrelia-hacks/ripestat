use serde::{Deserialize, Serialize};

pub mod resources;

#[doc(inline)]
pub use crate::resources::*;

/// RipeStatDataType
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RipeStatDataType {
    AbuseContactFinder,
    AddressSpaceHierarchy,
    AddressSpaceUsage,
    AllocationHistory,
    AnnouncedPrefixes,
    AsOverview,
    AsPathLength,
    AsRoutingConsistency,
    AsnNeighbours,
    AsnNeighboursHistory,
    AtlasProbeDeployment,
    AtlasProbes,
    AtlasTargets,
    BgpState,
    BgpUpdateActivity,
    BgpUpdates,
    Bgplay,
    CountryAsns,
    CountryResourceList,
    CountryResourceStats,
    DnsChain,
    HistoricalWhois,
    IanaRegistryInfo,
    LookingGlass,
    MaxmindGeoLite,
    MaxmindGeoLiteAnnouncedByAs,
    MeternetBandwidthMeasurements,
    MlabActivityCount,
    MlabBandwidth,
    MlabClients,
    NetworkInfo,
    PrefixCount,
    PrefixOverview,
    PrefixRoutingConsistency,
    PrefixSizeDistribution,
    RelatedPrefixes,
    ReverseDns,
    ReverseDnsConsistency,
    ReverseDnsIp,
    Rir,
    RirGeo,
    RirPrefixSizeDistribution,
    RirStatsCountry,
    RisAsns,
    RisFirstLastSeen,
    RisFullTableThreshold,
    RisPeerCount,
    RisPeerings,
    RisPeers,
    RisPrefixes,
    RoutingHistory,
    RoutingStatus,
    RpkiHistory,
    RpkiValidationStatus,
    Searchcomplete,
    SpeedcheckerBandwidthMeasurements,
    Visibility,
    Whois,
    WhoisObjectLastUpdated,
}

/// RipeStatDataType Url
impl RipeStatDataType {
    pub fn url(&self) -> String {
        let api_name: &str = match self {
            Self::AbuseContactFinder => "abuse-contact-finder",
            Self::AddressSpaceHierarchy => "address-space-hierarchy",
            Self::AddressSpaceUsage => "address-space-usage",
            Self::AllocationHistory => "allocation-history",
            Self::AnnouncedPrefixes => "announced-prefixes",
            Self::AsOverview => "as-overview",
            Self::AsPathLength => "as-path-length",
            Self::AsRoutingConsistency => "as-routing-consistency",
            Self::AsnNeighbours => "asn-neighbours",
            Self::AsnNeighboursHistory => "asn-neighbours-history",
            Self::AtlasProbeDeployment => "atlas-probe-deployment",
            Self::AtlasProbes => "atlas-probes",
            Self::AtlasTargets => "atlas-targets",
            Self::BgpState => "bgp-state",
            Self::BgpUpdateActivity => "bgp-update-activity",
            Self::BgpUpdates => "bgp-updates",
            Self::Bgplay => "bgplay",
            Self::CountryAsns => "country-asns",
            Self::CountryResourceList => "country-resource-list",
            Self::CountryResourceStats => "country-resource-stats",
            Self::DnsChain => "dns-chain",
            Self::HistoricalWhois => "historical-whois",
            Self::IanaRegistryInfo => "iana-registry-info",
            Self::LookingGlass => "looking-glass",
            Self::MaxmindGeoLite => "maxmind-geo-lite",
            Self::MaxmindGeoLiteAnnouncedByAs => "maxmind-geo-lite-announced-by-as",
            Self::MeternetBandwidthMeasurements => "meternet-bandwidth-measurements",
            Self::MlabActivityCount => "mlab-activity-count",
            Self::MlabBandwidth => "mlab-bandwidth",
            Self::MlabClients => "mlab-clients",
            Self::NetworkInfo => "network-info",
            Self::PrefixCount => "prefix-count",
            Self::PrefixOverview => "prefix-overview",
            Self::PrefixRoutingConsistency => "prefix-routing-consistency",
            Self::PrefixSizeDistribution => "prefix-size-distribution",
            Self::RelatedPrefixes => "related-prefixes",
            Self::ReverseDns => "reverse-dns",
            Self::ReverseDnsConsistency => "reverse-dns-consistency",
            Self::ReverseDnsIp => "reverse-dns-ip",
            Self::Rir => "rir",
            Self::RirGeo => "rir-geo",
            Self::RirPrefixSizeDistribution => "rir-prefix-size-distribution",
            Self::RirStatsCountry => "rir-stats-country",
            Self::RisAsns => "ris-asns",
            Self::RisFirstLastSeen => "ris-first-last-seen",
            Self::RisFullTableThreshold => "ris-full-table-threshold",
            Self::RisPeerCount => "ris-peer-count",
            Self::RisPeerings => "ris-peerings",
            Self::RisPeers => "ris-peers",
            Self::RisPrefixes => "ris-prefixes",
            Self::RoutingHistory => "routing-history",
            Self::RoutingStatus => "routing-status",
            Self::RpkiHistory => "rpki-history",
            Self::RpkiValidationStatus => "rpki-validation-status",
            Self::Searchcomplete => "searchcomplete",
            Self::SpeedcheckerBandwidthMeasurements => "speedchecker-bandwidth-measurements",
            Self::Visibility => "visibility",
            Self::Whois => "whois",
            Self::WhoisObjectLastUpdated => "whois-object-last-updated",
        };
        format!("https://stat.ripe.net/data/{}/data.json", api_name)
    }
}

/// RipeStatRequest
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RipeStatRequest {
    AbuseContactFinderRequest(AbuseContactFinderRequest),
    AddressSpaceHierarchyRequest(AddressSpaceHierarchyRequest),
    AddressSpaceUsageRequest(AddressSpaceUsageRequest),
    AllocationHistoryRequest(AllocationHistoryRequest),
    AnnouncedPrefixesRequest(AnnouncedPrefixesRequest),
    AsOverviewRequest(AsOverviewRequest),
    AsPathLengthRequest(AsPathLengthRequest),
    AsRoutingConsistencyRequest(AsRoutingConsistencyRequest),
    AsnNeighboursRequest(AsnNeighboursRequest),
    AsnNeighboursHistoryRequest(AsnNeighboursHistoryRequest),
    AtlasProbeDeploymentRequest(AtlasProbeDeploymentRequest),
    AtlasProbesRequest(AtlasProbesRequest),
    AtlasTargetsRequest(AtlasTargetsRequest),
    BgpStateRequest(BgpStateRequest),
    BgpUpdateActivityRequest(BgpUpdateActivityRequest),
    BgpUpdatesRequest(BgpUpdatesRequest),
    BgplayRequest(BgplayRequest),
    CountryAsnsRequest(CountryAsnsRequest),
    CountryResourceListRequest(CountryResourceListRequest),
    CountryResourceStatsRequest(CountryResourceStatsRequest),
    DnsChainRequest(DnsChainRequest),
    HistoricalWhoisRequest(HistoricalWhoisRequest),
    IanaRegistryInfoRequest(IanaRegistryInfoRequest),
    LookingGlassRequest(LookingGlassRequest),
    MaxmindGeoLiteRequest(MaxmindGeoLiteRequest),
    MaxmindGeoLiteAnnouncedByAsRequest(MaxmindGeoLiteAnnouncedByAsRequest),
    MeternetBandwidthMeasurementsRequest(MeternetBandwidthMeasurementsRequest),
    MlabActivityCountRequest(MlabActivityCountRequest),
    MlabBandwidthRequest(MlabBandwidthRequest),
    MlabClientsRequest(MlabClientsRequest),
    NetworkInfoRequest(NetworkInfoRequest),
    PrefixCountRequest(PrefixCountRequest),
    PrefixOverviewRequest(PrefixOverviewRequest),
    PrefixRoutingConsistencyRequest(PrefixRoutingConsistencyRequest),
    PrefixSizeDistributionRequest(PrefixSizeDistributionRequest),
    RelatedPrefixesRequest(RelatedPrefixesRequest),
    ReverseDnsRequest(ReverseDnsRequest),
    ReverseDnsConsistencyRequest(ReverseDnsConsistencyRequest),
    ReverseDnsIpRequest(ReverseDnsIpRequest),
    RirRequest(RirRequest),
    RirGeoRequest(RirGeoRequest),
    RirPrefixSizeDistributionRequest(RirPrefixSizeDistributionRequest),
    RirStatsCountryRequest(RirStatsCountryRequest),
    RisAsnsRequest(RisAsnsRequest),
    RisFirstLastSeenRequest(RisFirstLastSeenRequest),
    RisFullTableThresholdRequest(RisFullTableThresholdRequest),
    RisPeerCountRequest(RisPeerCountRequest),
    RisPeeringsRequest(RisPeeringsRequest),
    RisPeersRequest(RisPeersRequest),
    RisPrefixesRequest(RisPrefixesRequest),
    RoutingHistoryRequest(RoutingHistoryRequest),
    RoutingStatusRequest(RoutingStatusRequest),
    RpkiHistoryRequest(RpkiHistoryRequest),
    RpkiValidationStatusRequest(RpkiValidationStatusRequest),
    SearchcompleteRequest(SearchcompleteRequest),
    SpeedcheckerBandwidthMeasurementsRequest(SpeedcheckerBandwidthMeasurementsRequest),
    VisibilityRequest(VisibilityRequest),
    WhoisRequest(WhoisRequest),
    WhoisObjectLastUpdatedRequest(WhoisObjectLastUpdatedRequest),
}

/// RipeStatResponse
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RipeStatResponse {
    pub messages: Vec<Vec<String>>,
    pub see_also: Vec<Vec<String>>,

    pub build_version: String,
    pub cached: bool,
    pub data_call_name: RipeStatDataType,
    pub data_call_status: String,
    pub process_time: i64,
    pub query_id: String,
    pub server_id: String,
    pub status: String,
    pub status_code: i64,
    pub time: String,
    pub version: String,

    pub data: RipeStatResponseData,
}

impl<'de> Deserialize<'de> for RipeStatResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct RipeStatResponseInner {
            pub messages: Vec<Vec<String>>,
            pub see_also: Vec<Vec<String>>,
            pub build_version: String,
            pub cached: bool,
            pub data_call_name: RipeStatDataType,
            pub data_call_status: String,
            pub process_time: i64,
            pub query_id: String,
            pub server_id: String,
            pub status: String,
            pub status_code: i64,
            pub time: String,
            pub version: String,
            pub data: serde_json::Value,
        }

        let data = RipeStatResponseInner::deserialize(deserializer)?;

        let payload: RipeStatResponseData = match data.data_call_name {
            RipeStatDataType::AbuseContactFinder => RipeStatResponseData::AbuseContactFinderResponse(
                AbuseContactFinderResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AddressSpaceHierarchy => RipeStatResponseData::AddressSpaceHierarchyResponse(
                AddressSpaceHierarchyResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AddressSpaceUsage => RipeStatResponseData::AddressSpaceUsageResponse(
                AddressSpaceUsageResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AllocationHistory => RipeStatResponseData::AllocationHistoryResponse(
                AllocationHistoryResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AnnouncedPrefixes => RipeStatResponseData::AnnouncedPrefixesResponse(
                AnnouncedPrefixesResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AsOverview => RipeStatResponseData::AsOverviewResponse(
                AsOverviewResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AsPathLength => RipeStatResponseData::AsPathLengthResponse(
                AsPathLengthResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AsRoutingConsistency => RipeStatResponseData::AsRoutingConsistencyResponse(
                AsRoutingConsistencyResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AsnNeighbours => RipeStatResponseData::AsnNeighboursResponse(
                AsnNeighboursResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AsnNeighboursHistory => RipeStatResponseData::AsnNeighboursHistoryResponse(
                AsnNeighboursHistoryResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AtlasProbeDeployment => RipeStatResponseData::AtlasProbeDeploymentResponse(
                AtlasProbeDeploymentResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AtlasProbes => RipeStatResponseData::AtlasProbesResponse(
                AtlasProbesResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::AtlasTargets => RipeStatResponseData::AtlasTargetsResponse(
                AtlasTargetsResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::BgpState => RipeStatResponseData::BgpStateResponse(
                BgpStateResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::BgpUpdateActivity => RipeStatResponseData::BgpUpdateActivityResponse(
                BgpUpdateActivityResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::BgpUpdates => RipeStatResponseData::BgpUpdatesResponse(
                BgpUpdatesResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::Bgplay => RipeStatResponseData::BgplayResponse(
                BgplayResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::CountryAsns => RipeStatResponseData::CountryAsnsResponse(
                CountryAsnsResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::CountryResourceList => RipeStatResponseData::CountryResourceListResponse(
                CountryResourceListResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::CountryResourceStats => RipeStatResponseData::CountryResourceStatsResponse(
                CountryResourceStatsResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::DnsChain => RipeStatResponseData::DnsChainResponse(
                DnsChainResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::HistoricalWhois => RipeStatResponseData::HistoricalWhoisResponse(
                HistoricalWhoisResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::IanaRegistryInfo => RipeStatResponseData::IanaRegistryInfoResponse(
                IanaRegistryInfoResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::LookingGlass => RipeStatResponseData::LookingGlassResponse(
                LookingGlassResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::MaxmindGeoLite => RipeStatResponseData::MaxmindGeoLiteResponse(
                MaxmindGeoLiteResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::MaxmindGeoLiteAnnouncedByAs => RipeStatResponseData::MaxmindGeoLiteAnnouncedByAsResponse(
                MaxmindGeoLiteAnnouncedByAsResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::MeternetBandwidthMeasurements => RipeStatResponseData::MeternetBandwidthMeasurementsResponse(
                MeternetBandwidthMeasurementsResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::MlabActivityCount => RipeStatResponseData::MlabActivityCountResponse(
                MlabActivityCountResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::MlabBandwidth => RipeStatResponseData::MlabBandwidthResponse(
                MlabBandwidthResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::MlabClients => RipeStatResponseData::MlabClientsResponse(
                MlabClientsResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::NetworkInfo => RipeStatResponseData::NetworkInfoResponse(
                NetworkInfoResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::PrefixCount => RipeStatResponseData::PrefixCountResponse(
                PrefixCountResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::PrefixOverview => RipeStatResponseData::PrefixOverviewResponse(
                PrefixOverviewResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::PrefixRoutingConsistency => RipeStatResponseData::PrefixRoutingConsistencyResponse(
                PrefixRoutingConsistencyResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::PrefixSizeDistribution => RipeStatResponseData::PrefixSizeDistributionResponse(
                PrefixSizeDistributionResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RelatedPrefixes => RipeStatResponseData::RelatedPrefixesResponse(
                RelatedPrefixesResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::ReverseDns => RipeStatResponseData::ReverseDnsResponse(
                ReverseDnsResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::ReverseDnsConsistency => RipeStatResponseData::ReverseDnsConsistencyResponse(
                ReverseDnsConsistencyResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::ReverseDnsIp => RipeStatResponseData::ReverseDnsIpResponse(
                ReverseDnsIpResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::Rir => RipeStatResponseData::RirResponse(
                RirResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RirGeo => RipeStatResponseData::RirGeoResponse(
                RirGeoResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RirPrefixSizeDistribution => RipeStatResponseData::RirPrefixSizeDistributionResponse(
                RirPrefixSizeDistributionResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RirStatsCountry => RipeStatResponseData::RirStatsCountryResponse(
                RirStatsCountryResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RisAsns => RipeStatResponseData::RisAsnsResponse(
                RisAsnsResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RisFirstLastSeen => RipeStatResponseData::RisFirstLastSeenResponse(
                RisFirstLastSeenResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RisFullTableThreshold => RipeStatResponseData::RisFullTableThresholdResponse(
                RisFullTableThresholdResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RisPeerCount => RipeStatResponseData::RisPeerCountResponse(
                RisPeerCountResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RisPeerings => RipeStatResponseData::RisPeeringsResponse(
                RisPeeringsResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RisPeers => RipeStatResponseData::RisPeersResponse(
                RisPeersResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RisPrefixes => RipeStatResponseData::RisPrefixesResponse(
                RisPrefixesResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RoutingHistory => RipeStatResponseData::RoutingHistoryResponse(
                RoutingHistoryResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RoutingStatus => RipeStatResponseData::RoutingStatusResponse(
                RoutingStatusResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RpkiHistory => RipeStatResponseData::RpkiHistoryResponse(
                RpkiHistoryResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::RpkiValidationStatus => RipeStatResponseData::RpkiValidationStatusResponse(
                RpkiValidationStatusResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::Searchcomplete => RipeStatResponseData::SearchcompleteResponse(
                SearchcompleteResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::SpeedcheckerBandwidthMeasurements => RipeStatResponseData::SpeedcheckerBandwidthMeasurementsResponse(
                SpeedcheckerBandwidthMeasurementsResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::Visibility => RipeStatResponseData::VisibilityResponse(
                VisibilityResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::Whois => RipeStatResponseData::WhoisResponse(
                WhoisResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
            RipeStatDataType::WhoisObjectLastUpdated => RipeStatResponseData::WhoisObjectLastUpdatedResponse(
                WhoisObjectLastUpdatedResponse::deserialize(data.data).map_err(serde::de::Error::custom)?,
            ),
        };

        Ok(RipeStatResponse {
            messages: data.messages,
            see_also: data.see_also,
            build_version: data.build_version,
            cached: data.cached,
            data_call_name: data.data_call_name,
            data_call_status: data.data_call_status,
            process_time: data.process_time,
            query_id: data.query_id,
            server_id: data.server_id,
            status: data.status,
            status_code: data.status_code,
            time: data.time,
            version: data.version,
            data: payload,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum RipeStatResponseData {
    AbuseContactFinderResponse(AbuseContactFinderResponse),
    AddressSpaceHierarchyResponse(AddressSpaceHierarchyResponse),
    AddressSpaceUsageResponse(AddressSpaceUsageResponse),
    AllocationHistoryResponse(AllocationHistoryResponse),
    AnnouncedPrefixesResponse(AnnouncedPrefixesResponse),
    AsOverviewResponse(AsOverviewResponse),
    AsPathLengthResponse(AsPathLengthResponse),
    AsRoutingConsistencyResponse(AsRoutingConsistencyResponse),
    AsnNeighboursResponse(AsnNeighboursResponse),
    AsnNeighboursHistoryResponse(AsnNeighboursHistoryResponse),
    AtlasProbeDeploymentResponse(AtlasProbeDeploymentResponse),
    AtlasProbesResponse(AtlasProbesResponse),
    AtlasTargetsResponse(AtlasTargetsResponse),
    BgpStateResponse(BgpStateResponse),
    BgpUpdateActivityResponse(BgpUpdateActivityResponse),
    BgpUpdatesResponse(BgpUpdatesResponse),
    BgplayResponse(BgplayResponse),
    CountryAsnsResponse(CountryAsnsResponse),
    CountryResourceListResponse(CountryResourceListResponse),
    CountryResourceStatsResponse(CountryResourceStatsResponse),
    DnsChainResponse(DnsChainResponse),
    HistoricalWhoisResponse(HistoricalWhoisResponse),
    IanaRegistryInfoResponse(IanaRegistryInfoResponse),
    LookingGlassResponse(LookingGlassResponse),
    MaxmindGeoLiteResponse(MaxmindGeoLiteResponse),
    MaxmindGeoLiteAnnouncedByAsResponse(MaxmindGeoLiteAnnouncedByAsResponse),
    MeternetBandwidthMeasurementsResponse(MeternetBandwidthMeasurementsResponse),
    MlabActivityCountResponse(MlabActivityCountResponse),
    MlabBandwidthResponse(MlabBandwidthResponse),
    MlabClientsResponse(MlabClientsResponse),
    NetworkInfoResponse(NetworkInfoResponse),
    PrefixCountResponse(PrefixCountResponse),
    PrefixOverviewResponse(PrefixOverviewResponse),
    PrefixRoutingConsistencyResponse(PrefixRoutingConsistencyResponse),
    PrefixSizeDistributionResponse(PrefixSizeDistributionResponse),
    RelatedPrefixesResponse(RelatedPrefixesResponse),
    ReverseDnsResponse(ReverseDnsResponse),
    ReverseDnsConsistencyResponse(ReverseDnsConsistencyResponse),
    ReverseDnsIpResponse(ReverseDnsIpResponse),
    RirResponse(RirResponse),
    RirGeoResponse(RirGeoResponse),
    RirPrefixSizeDistributionResponse(RirPrefixSizeDistributionResponse),
    RirStatsCountryResponse(RirStatsCountryResponse),
    RisAsnsResponse(RisAsnsResponse),
    RisFirstLastSeenResponse(RisFirstLastSeenResponse),
    RisFullTableThresholdResponse(RisFullTableThresholdResponse),
    RisPeerCountResponse(RisPeerCountResponse),
    RisPeeringsResponse(RisPeeringsResponse),
    RisPeersResponse(RisPeersResponse),
    RisPrefixesResponse(RisPrefixesResponse),
    RoutingHistoryResponse(RoutingHistoryResponse),
    RoutingStatusResponse(RoutingStatusResponse),
    RpkiHistoryResponse(RpkiHistoryResponse),
    RpkiValidationStatusResponse(RpkiValidationStatusResponse),
    SearchcompleteResponse(SearchcompleteResponse),
    SpeedcheckerBandwidthMeasurementsResponse(SpeedcheckerBandwidthMeasurementsResponse),
    VisibilityResponse(VisibilityResponse),
    WhoisResponse(WhoisResponse),
    WhoisObjectLastUpdatedResponse(WhoisObjectLastUpdatedResponse),
}

/// Version of this software.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");