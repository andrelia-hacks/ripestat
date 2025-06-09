use thiserror::Error;

pub mod client;


/// Error returned by RIPEstat client functions and methods.
#[derive(Error, Debug)]
pub enum RipestatClientError {
    #[error(transparent)]
    Client(#[from] reqwest::Error),
}