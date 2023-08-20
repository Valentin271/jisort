//! This module groups error related data.
//!
//! Mainly it holds a wrapper [Error] type.

use std::{fmt::Display, io};

/// This is just a wrapper around different libraries errors.
#[derive(Debug)]
pub enum Error {
    /// Errors from the [ignore] library.
    Ignore(ignore::Error),
    /// Errors from [std::io::Error].
    Io(io::Error),

    /// Jsort generic error
    Jsort {
        /// Text describing the error
        err: String,
    },
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jsort { err } => {
                write!(f, "\x1b[31mjisort error:\x1b[m {}", err)
            }
            Error::Ignore(err) => err.fmt(f),
            Error::Io(err) => err.fmt(f),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Jsort {
            err: value.to_owned(),
        }
    }
}

impl From<ignore::Error> for Error {
    fn from(value: ignore::Error) -> Self {
        Self::Ignore(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
