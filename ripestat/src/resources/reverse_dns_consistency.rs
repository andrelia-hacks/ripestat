use reqwest;

use crate::{client, RipestatClientError};

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::ReverseDnsConsistencyRequest;

pub async fn reverse_dns_consistency(request: ReverseDnsConsistencyRequest, client: &client::Client) -> Result<RipeStatResponse, RipestatClientError> {
    let data_type = RipeStatDataType::ReverseDnsConsistency;
    let url = data_type.url();
    let url_string = url.as_str();

    let mut param_vec: Vec<(String, String)> = vec![
        ("resource".to_owned(), request.resource)
    ];
    if let Some(v) = request.ipv4 {
        param_vec.push(("ipv4".to_owned(), v.to_string()));
    }
    if let Some(v) = request.ipv6 {
        param_vec.push(("ipv6".to_owned(), v.to_string()));
    }
    let params = param_vec.iter();

    let request_uri = reqwest::Url::parse_with_params(url_string, params)?;

    let wrapped_response = client::wrapped_request(request_uri, &client).await?;

    let ripe_stat_response: RipeStatResponse = serde_json::from_str(&wrapped_response.text)?;
    
    Ok(ripe_stat_response)
}
