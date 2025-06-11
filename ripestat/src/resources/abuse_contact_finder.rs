use reqwest;
use reqwest::header::USER_AGENT;

use crate::RipeStatClientError;

use ripestat_common::{RipeStatDataType, RipeStatResponse};
use ripestat_common::AbuseContactFinderRequest;

pub async fn abuse_contact_finder(request: AbuseContactFinderRequest) -> Result<RipeStatResponse, RipeStatClientError> {
    let data_type = RipeStatDataType::AbuseContactFinder;
    let url = data_type.url();
    let url_string = url.as_str();
    println!("{}", &url_string);

    let param_vec: Vec<(String, String)> = vec![("resource".to_owned(), request.resource)];
    println!("{:?}", &param_vec);
    let params = param_vec.iter();

    let request_url = reqwest::Url::parse_with_params(url_string, params).unwrap();
    println!("{}", &request_url);

    let client = reqwest::Client::new();
    let response = match client
        .get(request_url)
        .header(USER_AGENT, "ripestat/0.1")
        .send()
        .await {
            Ok(i) => i,
            Err(e) => return Err(RipeStatClientError::Client(e))
        };

    let ripe_stat_response: RipeStatResponse = match response.json().await {
        Ok(i) => i,
        Err(e) => return Err(RipeStatClientError::Client(e))
    };
    
    Ok(ripe_stat_response)
}
