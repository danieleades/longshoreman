//! Representations of various client errors

use http;
use hyper::StatusCode;
use serde_json::Error as SerdeError;
use std::{error::Error as StdError, fmt, io::Error as IoError, string::FromUtf8Error};
use tokio_util::codec::{LengthDelimitedCodecError, LinesCodecError};

/// Represents the result of all docker operations
pub type Result<T> = std::result::Result<T, Error>;

/// A 'catch-all' error for anything that can go wrong with this crate.
#[derive(Debug)]
pub enum Error {
    #[doc(hidden)]
    SerdeJson(SerdeError),
    /// Errors from the underlying Hyper crate
    Hyper(hyper::Error),

    /// Low-level Http errors
    Http(http::Error),

    /// Errors related to file I/O from the host OS
    IO(IoError),
    #[doc(hidden)]
    Encoding(FromUtf8Error),

    /// An invalid response form the docker API
    InvalidResponse(String),

    /// A canonical Http error response
    Fault {
        /// The canonical HTTP status code
        code: StatusCode,

        /// A descriptive string
        message: String,
    },

    /// An error which occurs when an http connection fails to upgrade to TCP on
    /// request
    ConnectionNotUpgraded,

    #[doc(hidden)]
    Decode,
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Error {
        Error::SerdeJson(error)
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Error {
        Error::Hyper(error)
    }
}

impl From<http::Error> for Error {
    fn from(error: http::Error) -> Error {
        Error::Http(error)
    }
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

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::IO(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::Encoding(error)
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Docker Error: ")?;
        match self {
            Error::SerdeJson(err) => write!(f, "{}", err),
            Error::Http(ref err) => write!(f, "{}", err),
            Error::Hyper(ref err) => write!(f, "{}", err),
            Error::IO(ref err) => write!(f, "{}", err),
            Error::Encoding(ref err) => write!(f, "{}", err),
            Error::InvalidResponse(ref cause) => {
                write!(f, "Response doesn't have the expected format: {}", cause)
            }
            Error::Fault { code, .. } => write!(f, "{}", code),
            Error::ConnectionNotUpgraded => write!(
                f,
                "expected the docker host to upgrade the HTTP connection but it did not"
            ),
            Error::Decode => write!(f, "failed to decode bytes"),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            Error::SerdeJson(ref err) => Some(err),
            Error::Http(ref err) => Some(err),
            Error::IO(ref err) => Some(err),
            Error::Encoding(e) => Some(e),
            _ => None,
        }
    }
}
