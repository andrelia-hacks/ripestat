use thiserror::Error;

pub mod client;
pub mod resources;


#[derive(Debug, Error)]
pub enum RipestatClientError {
    /// An error has occurred parsing the JSON.
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    /// An error has occurred during the request
    #[error(transparent)]
    Client(#[from] reqwest::Error),

    #[error(transparent)]
    UrlParsing(#[from] url::ParseError),
}