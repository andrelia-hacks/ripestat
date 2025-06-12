use reqwest;

use crate::{client, RipestatClientError};

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::RpkiHistoryRequest;

pub async fn rpki_history(request: RpkiHistoryRequest, client: &client::Client) -> Result<RipeStatResponse, RipestatClientError> {
    let data_type = RipeStatDataType::RpkiHistory;
    let url = data_type.url();
    let url_string = url.as_str();

    let mut param_vec: Vec<(String, String)> = vec![
        ("resource".to_owned(), request.resource)
    ];
    if let Some(v) = request.family {
        param_vec.push(("family".to_owned(), v.to_string()));
    }
    if let Some(v) = request.resolution {
        param_vec.push(("resolution".to_owned(), v));
    }
    if let Some(v) = request.include {
        param_vec.push(("include".to_owned(), v));
    }
    if let Some(v) = request.delegated {
        param_vec.push(("delegated".to_owned(), v.to_string()));
    }
    let params = param_vec.iter();

    let request_uri = reqwest::Url::parse_with_params(url_string, params)?;

    let wrapped_response = client::wrapped_request(request_uri, &client).await?;

    let ripe_stat_response: RipeStatResponse = serde_json::from_str(&wrapped_response.text)?;
    
    Ok(ripe_stat_response)
}
