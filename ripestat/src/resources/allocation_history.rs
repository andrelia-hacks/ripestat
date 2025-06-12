use reqwest;

use crate::{client, RipestatClientError};

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::AllocationHistoryRequest;

pub async fn allocation_history(request: AllocationHistoryRequest, client: &client::Client) -> Result<RipeStatResponse, RipestatClientError> {
    let data_type = RipeStatDataType::AllocationHistory;
    let url = data_type.url();
    let url_string = url.as_str();

    let mut param_vec: Vec<(String, String)> = vec![
        ("resource".to_owned(), request.resource)
    ];
    match request.starttime {
        Some(v) => param_vec.push(("starttime".to_owned(), v)),
        None => (),
    }
    match request.endtime {
        Some(v) => param_vec.push(("endtime".to_owned(), v)),
        None => (),
    }
    let params = param_vec.iter();

    let request_uri = reqwest::Url::parse_with_params(url_string, params)?;

    let wrapped_response = client::wrapped_request(request_uri, &client).await?;

    let ripe_stat_response: RipeStatResponse = serde_json::from_str(&wrapped_response.text)?;
    
    Ok(ripe_stat_response)
}
