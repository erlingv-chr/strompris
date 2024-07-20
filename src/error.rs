//! The module containing errors for this crate

use derive_more::{Display, From};
use url::ParseError;

/// Mapping the built-in Result to a blanket signature that always uses this crate's Error
pub type Result<T> = std::result::Result<T, Error>;

/// The errors used within this crate
#[derive(Debug, From, Display)]
pub enum Error {
    /// Errors that does not originate from dependencies
    #[from]
    Generic(String),
    /// Wrapper for reqwest::Error
    #[from]
    ReqwestError(reqwest::Error),
    /// Wrapper for url::ParseError
    #[from]
    ParseError(ParseError),
}
