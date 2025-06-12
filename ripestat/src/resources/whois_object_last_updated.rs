use reqwest;

use crate::{client, RipestatClientError};

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::WhoisObjectLastUpdatedRequest;

pub async fn whois_object_last_updated(request: WhoisObjectLastUpdatedRequest, client: &client::Client) -> Result<RipeStatResponse, RipestatClientError> {
    let data_type = RipeStatDataType::WhoisObjectLastUpdated;
    let url = data_type.url();
    let url_string = url.as_str();

    let mut param_vec: Vec<(String, String)> = vec![
        ("object".to_owned(), request.object),
        ("type".to_owned(), request.object_type),
        ("source".to_owned(), request.source),
    ];
    if let Some(v) = request.timestamp {
        param_vec.push(("timestamp".to_owned(), v));
    }
    if let Some(v) = request.compare_with_live {
        param_vec.push(("compare_with_live".to_owned(), v.to_string()));
    }
    let params = param_vec.iter();

    let request_uri = reqwest::Url::parse_with_params(url_string, params)?;

    let wrapped_response = client::wrapped_request(request_uri, &client).await?;

    let ripe_stat_response: RipeStatResponse = serde_json::from_str(&wrapped_response.text)?;
    
    Ok(ripe_stat_response)
}
