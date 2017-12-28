use serde_json::Error as JsonError;
use std::io::Error as IoError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

#[cfg(feature = "hyper-support")]
use hyper::error::{Error as HyperError, UriError};
#[cfg(feature = "reqwest-support")]
use reqwest::Error as ReqwestError;

/// Common result type used throughout the library.
pub type Result<T> = StdResult<T, Error>;

/// Common error type used throughout the library, to be used as a holder for
/// errors from various other libraries.
#[derive(Debug)]
pub enum Error {
    /// A `hyper` crate error.
    #[cfg(feature = "hyper-support")]
    Hyper(HyperError),
    /// A `serde_json` crate error.
    Json(JsonError),
    /// A `std::io` module error.
    Io(IoError),
    /// An error from the `reqwest` crate.
    #[cfg(feature = "reqwest-support")]
    Reqwest(ReqwestError),
    /// An error from `hyper` while parsing a URI.
    #[cfg(feature = "hyper-support")]
    Uri(UriError),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

#[cfg(feature = "hyper-support")]
impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Error::Hyper(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

#[cfg(feature = "reqwest-support")]
impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Error {
        Error::Reqwest(err)
    }
}

#[cfg(feature = "hyper-support")]
impl From<UriError> for Error {
    fn from(err: UriError) -> Error {
        Error::Uri(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            #[cfg(feature = "hyper-support")]
            Error::Hyper(ref inner) => inner.fmt(f),
            Error::Json(ref inner) => inner.fmt(f),
            Error::Io(ref inner) => inner.fmt(f),
            #[cfg(feature = "reqwest-support")]
            Error::Reqwest(ref inner) => inner.fmt(f),
            #[cfg(feature = "hyper-support")]
            Error::Uri(ref inner) => inner.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            #[cfg(feature = "hyper-support")]
            Error::Hyper(ref inner) => inner.description(),
            Error::Json(ref inner) => inner.description(),
            Error::Io(ref inner) => inner.description(),
            #[cfg(feature = "reqwest-support")]
            Error::Reqwest(ref inner) => inner.description(),
            #[cfg(feature = "hyper-support")]
            Error::Uri(ref inner) => inner.description(),
        }
    }
}
