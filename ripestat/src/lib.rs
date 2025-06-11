use thiserror::Error;

pub mod resources;


/// Error returned by RIPEstat client functions and methods.
#[derive(Error, Debug)]
pub enum RipeStatClientError {
    #[error(transparent)]
    Client(#[from] reqwest::Error),
}