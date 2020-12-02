use std::char::ParseCharError;
use std::fmt::Debug;
use std::io;
use std::num::ParseIntError;
use thiserror::Error;

/// Methods are passed input as a line iterator on the input file
pub type Input = Box<dyn Iterator<Item = io::Result<String>>>;

/// Errors caused by invalid client input
#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("No match found")]
    NoMatch(),
    #[error("{0}")]
    FromString(String),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    ParseCharError(#[from] ParseCharError),
}
