use reqwest;

use crate::{client, RipestatClientError};

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::PrefixOverviewRequest;

pub async fn prefix_overview(request: PrefixOverviewRequest, client: &client::Client) -> Result<RipeStatResponse, RipestatClientError> {
    let data_type = RipeStatDataType::PrefixOverview;
    let url = data_type.url();
    let url_string = url.as_str();

    let mut param_vec: Vec<(String, String)> = vec![
        ("resource".to_owned(), request.resource)
    ];
    if let Some(v) = request.min_peers_seeing {
        param_vec.push(("min_peers_seeing".to_owned(), v.to_string()));
    }
    if let Some(v) = request.max_related {
        param_vec.push(("max_related".to_owned(), v.to_string()));
    }
    if let Some(v) = request.query_time {
        param_vec.push(("query_time".to_owned(), v));
    }
    let params = param_vec.iter();

    let request_uri = reqwest::Url::parse_with_params(url_string, params)?;

    let wrapped_response = client::wrapped_request(request_uri, &client).await?;

    let ripe_stat_response: RipeStatResponse = serde_json::from_str(&wrapped_response.text)?;
    
    Ok(ripe_stat_response)
}
