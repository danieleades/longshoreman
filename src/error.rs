//! Representations of various client errors

use hyper::StatusCode;
use std::{io, string::FromUtf8Error};
use tokio_util::codec::{LengthDelimitedCodecError, LinesCodecError};

/// Represents the result of all docker operations
pub type Result<T> = std::result::Result<T, Error>;

/// A 'catch-all' error for anything that can go wrong with this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that occur when serialising/deserialising from JSON
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    /// Errors from the underlying Hyper crate
    #[error(transparent)]
    Hyper(#[from] hyper::Error),

    /// Low-level Http errors
    #[error(transparent)]
    Http(#[from] http::Error),

    /// Errors related to file I/O from the host OS
    #[error(transparent)]
    IO(#[from] io::Error),

    /// Utf8 encoding errors
    #[error(transparent)]
    Encoding(#[from] FromUtf8Error),

    /// invalid response error
    #[error("Response doesn't have the expected format: {0}")]
    InvalidResponse(String),

    /// Canonical HTTP errors
    #[error("{code}: {message}")]
    Fault {
        /// The canonical HTTP status code
        code: StatusCode,

        /// A descriptive string
        message: String,
    },

    /// Error when the docker host fails to upgrade the HTTP connection
    #[error("expected the docker host to upgrade the HTTP connection but it did not")]
    ConnectionNotUpgraded,

    /// Errors that occur when decoding byte streams
    #[error("failed to decode bytes")]
    Decode,
}

impl From<http::uri::InvalidUri> for Error {
    fn from(error: http::uri::InvalidUri) -> Self {
        let http_error: http::Error = error.into();
        http_error.into()
    }
}

impl From<http::header::InvalidHeaderValue> for Error {
    fn from(error: http::header::InvalidHeaderValue) -> Self {
        let http_error = http::Error::from(error);
        http_error.into()
    }
}

impl From<LinesCodecError> for Error {
    fn from(error: LinesCodecError) -> Self {
        match error {
            LinesCodecError::MaxLineLengthExceeded => Self::Decode,
            LinesCodecError::Io(e) => Self::IO(e),
        }
    }
}

impl From<LengthDelimitedCodecError> for Error {
    fn from(_error: LengthDelimitedCodecError) -> Self {
        Self::Decode
    }
}
