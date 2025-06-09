use ripestat::{client::request::get_as_overview, RipestatClientError};
use ripestat_common::response::as_overview::AsOverview;

#[tokio::main]
async fn main() -> Result<(), RipestatClientError> {
    let asn: String = "3333".to_owned();
    let as_overview: AsOverview = get_as_overview(asn).await?;
    println!("{:#?}", as_overview);

    Ok(())
}
