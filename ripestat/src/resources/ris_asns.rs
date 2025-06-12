use reqwest;

use crate::{client, RipestatClientError};

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::RisAsnsRequest;

pub async fn ris_asns(request: RisAsnsRequest, client: &client::Client) -> Result<RipeStatResponse, RipestatClientError> {
    let data_type = RipeStatDataType::RisAsns;
    let url = data_type.url();
    let url_string = url.as_str();

    let mut param_vec: Vec<(String, String)> = vec![
        ("query_time".to_owned(), request.query_time)
    ];
    if let Some(v) = request.list_asns {
        param_vec.push(("list_asns".to_owned(), v.to_string()));
    }
    if let Some(v) = request.asn_types {
        param_vec.push(("asn_types".to_owned(), v));
    }
    let params = param_vec.iter();

    let request_uri = reqwest::Url::parse_with_params(url_string, params)?;

    let wrapped_response = client::wrapped_request(request_uri, &client).await?;

    let ripe_stat_response: RipeStatResponse = serde_json::from_str(&wrapped_response.text)?;
    
    Ok(ripe_stat_response)
}
