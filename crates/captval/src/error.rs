//! Error types for captval

use std::collections::HashSet;
use std::io;
use thiserror::Error;

use crate::Code;

/// The error type for captval.
/// Provides error types to capture error codes from the
/// captcha APIs and errors output from crates used by the
/// library.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// Error(s) returned from the captval API and mapped to the [Code] enum.
    #[error("{0:?}")]
    Codes(HashSet<Code>),
    /// Error returned by reqwest
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    /// Error returned by io
    #[error("{0}")]
    Io(#[from] io::Error),
    /// Error returned by serde_json
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    /// Error returned by serde_urlencoded
    #[error("{0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),
    /// Error returned by uuid
    #[error("{0}")]
    Uuid(#[from] uuid::Error),
    /// Error returned by url parser
    #[error("{0}")]
    Url(#[from] url::ParseError),
}
