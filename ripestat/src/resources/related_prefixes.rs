use reqwest;

use crate::{client, RipestatClientError};

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::RelatedPrefixesRequest;

pub async fn related_prefixes(request: RelatedPrefixesRequest, client: &client::Client) -> Result<RipeStatResponse, RipestatClientError> {
    let data_type = RipeStatDataType::RelatedPrefixes;
    let url = data_type.url();
    let url_string = url.as_str();

    let param_vec: Vec<(String, String)> = vec![
        ("resource".to_owned(), request.resource)
    ];
    let params = param_vec.iter();

    let request_uri = reqwest::Url::parse_with_params(url_string, params)?;

    let wrapped_response = client::wrapped_request(request_uri, &client).await?;

    let ripe_stat_response: RipeStatResponse = serde_json::from_str(&wrapped_response.text)?;
    
    Ok(ripe_stat_response)
}
