//! Representations of various client errors

use http;
use hyper::{self, StatusCode};
use serde_json::Error as SerdeError;
use std::{error::Error as StdError, fmt, io::Error as IoError, string::FromUtf8Error};
use tokio_util::codec::{LengthDelimitedCodecError, LinesCodecError};

/// Represents the result of all docker operations
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SerdeJsonError(SerdeError),
    Hyper(hyper::Error),
    Http(http::Error),
    IO(IoError),
    Encoding(FromUtf8Error),
    InvalidResponse(String),
    Fault { code: StatusCode, message: String },
    ConnectionNotUpgraded,
    Decode,
}

impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Error {
        Error::SerdeJsonError(error)
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
            Error::SerdeJsonError(err) => write!(f, "{}", err),
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
            Error::SerdeJsonError(ref err) => Some(err),
            Error::Http(ref err) => Some(err),
            Error::IO(ref err) => Some(err),
            Error::Encoding(e) => Some(e),
            _ => None,
        }
    }
}
