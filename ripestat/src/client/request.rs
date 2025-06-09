use reqwest;
use reqwest::header::USER_AGENT;

use crate::RipestatClientError;

use ripestat_common::datatype::RipeStatDataType;
use ripestat_common::response::as_overview::AsOverview;

pub async fn get_as_overview(query: String) -> Result<AsOverview, RipestatClientError> {
    let data_type: RipeStatDataType = RipeStatDataType::ASOverview;

    let request_url = format!("{data_url}?resource={resource}",
        data_url = data_type.url(),
        resource = query
    );

    let client = reqwest::Client::new();
    let response = match client
        .get(request_url)
        .header(USER_AGENT, "ripestat/0.1")
        .send()
        .await {
            Ok(i) => i,
            Err(e) => return Err(RipestatClientError::Client(e))
        };

    let response_data: AsOverview = match response.json().await {
        Ok(i) => i,
        Err(e) => return Err(RipestatClientError::Client(e))
    };
    
    Ok(response_data)
}
