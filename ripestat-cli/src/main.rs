use ripestat::{
    resources::abuse_contact_finder,
    client::{self},
    RipestatClientError,
};
use ripestat_common::{
    AbuseContactFinderRequest,
    RipeStatResponse,
};

#[tokio::main]
async fn main() -> Result<(), RipestatClientError> {
    let config = client::ClientConfig::default();
    let client = client::create_client(&config)?;

    let request: AbuseContactFinderRequest = AbuseContactFinderRequest {
        resource: "3333".to_owned(),
    };
    let ripe_stat_response: RipeStatResponse = abuse_contact_finder(request, &client).await?;
    println!("{:#?}", ripe_stat_response);

    Ok(())
}
