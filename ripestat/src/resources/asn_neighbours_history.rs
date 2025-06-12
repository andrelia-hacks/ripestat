use reqwest;

use crate::{client, RipestatClientError};

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::AsnNeighboursHistoryRequest;

pub async fn asn_neighbours_history(request: AsnNeighboursHistoryRequest, client: &client::Client) -> Result<RipeStatResponse, RipestatClientError> {
    let data_type = RipeStatDataType::AsnNeighboursHistory;
    let url = data_type.url();
    let url_string = url.as_str();

    let mut param_vec: Vec<(String, String)> = vec![
        ("resource".to_owned(), request.resource)
    ];
    if let Some(v) = request.starttime {
        param_vec.push(("starttime".to_owned(), v));
    }
    if let Some(v) = request.endtime {
        param_vec.push(("endtime".to_owned(), v));
    }
    if let Some(v) = request.max_rows {
        param_vec.push(("max_rows".to_owned(), v.to_string()));
    }
    let params = param_vec.iter();

    let request_uri = reqwest::Url::parse_with_params(url_string, params)?;

    let wrapped_response = client::wrapped_request(request_uri, &client).await?;

    let ripe_stat_response: RipeStatResponse = serde_json::from_str(&wrapped_response.text)?;
    
    Ok(ripe_stat_response)
}
