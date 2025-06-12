use reqwest;

use crate::{client, RipestatClientError};

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::LookingGlassRequest;

pub async fn looking_glass(request: LookingGlassRequest, client: &client::Client) -> Result<RipeStatResponse, RipestatClientError> {
    let data_type = RipeStatDataType::LookingGlass;
    let url = data_type.url();
    let url_string = url.as_str();

    let mut param_vec: Vec<(String, String)> = vec![
        ("resource".to_owned(), request.resource)
    ];
    if let Some(v) = request.look_back_limit {
        param_vec.push(("look_back_limit".to_owned(), v));
    }
    let params = param_vec.iter();

    let request_uri = reqwest::Url::parse_with_params(url_string, params)?;

    let wrapped_response = client::wrapped_request(request_uri, &client).await?;

    let ripe_stat_response: RipeStatResponse = serde_json::from_str(&wrapped_response.text)?;
    
    Ok(ripe_stat_response)
}
