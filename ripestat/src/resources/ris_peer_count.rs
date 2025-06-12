use reqwest;

use crate::{client, RipestatClientError};

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::RisPeerCountRequest;

pub async fn ris_peer_count(request: RisPeerCountRequest, client: &client::Client) -> Result<RipeStatResponse, RipestatClientError> {
    let data_type = RipeStatDataType::RisPeerCount;
    let url = data_type.url();
    let url_string = url.as_str();

    let mut param_vec: Vec<(String, String)> = vec![];
    if let Some(v) = request.starttime {
        param_vec.push(("starttime".to_owned(), v));
    }
    if let Some(v) = request.endtime {
        param_vec.push(("endtime".to_owned(), v));
    }
    if let Some(v) = request.v4_full_prefix_threshold {
        param_vec.push(("v4_full_prefix_threshold".to_owned(), v.to_string()));
    }
    if let Some(v) = request.v6_full_prefix_threshold {
        param_vec.push(("v6_full_prefix_threshold".to_owned(), v.to_string()));
    }
    let params = param_vec.iter();

    let request_uri = reqwest::Url::parse_with_params(url_string, params)?;

    let wrapped_response = client::wrapped_request(request_uri, &client).await?;

    let ripe_stat_response: RipeStatResponse = serde_json::from_str(&wrapped_response.text)?;
    
    Ok(ripe_stat_response)
}
