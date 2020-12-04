use std::char::ParseCharError;
use std::fmt::Debug;
use std::io;
use std::num::ParseIntError;
use thiserror::Error;

/// Methods are passed input as a line iterator on the input file
pub type Input = Box<dyn Iterator<Item = io::Result<String>>>;

/// Errors caused by invalid client input
#[derive(Error, Debug)]
pub enum Error {
    #[error("No match found")]
    NoMatch(),
    #[error("Lookup out {0} of bounds, len {1}")]
    OutOfBounds(usize, usize),
    #[error("{0}")]
    FromString(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    ParseCharError(#[from] ParseCharError),
}

#[macro_export]
macro_rules! return_error {
    ($($arg:tt)*) => {{
        return Err(Error::FromString(format![$($arg)*]))
    }}
}
