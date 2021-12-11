use anyhow::{bail, Result};
use regex::Regex;
use std::str::FromStr;

/// Helper trait to work with regexp captures with less boilerplate
pub trait CaptureParser {
    /// Extract into a string or bail if the capture is empty
    fn try_get(&self, index: usize) -> Result<&str>;

    /// Parse any regexp capture into a type implementing FromStr
    fn parse<T: FromStr>(&self, index: usize) -> Result<T>
    where
        <T as std::str::FromStr>::Err: std::error::Error;

    /// Extract the value and checks it against a regexp
    fn matches(&self, index: usize, regexp: &Regex) -> bool;
}

impl CaptureParser for regex::Captures<'_> {
    fn try_get(&self, index: usize) -> Result<&str> {
        match self.get(index) {
            None => bail!("Empty input"),
            Some(value) => Ok(value.as_str()),
        }
    }

    fn parse<T: FromStr>(&self, index: usize) -> Result<T>
    where
        <T as std::str::FromStr>::Err: std::error::Error,
    {
        match T::from_str(self.try_get(index)?) {
            Ok(value) => Ok(value),
            Err(err) => bail!("Conversion error: {}", err),
        }
    }

    fn matches(&self, index: usize, regexp: &Regex) -> bool {
        match self.get(index) {
            None => false,
            Some(value) => regexp.is_match(value.as_str()),
        }
    }
}

/// Parses a line of ASCII digits into a vector of u8 integers, fails on non-ascii input.
/// Non-digit ASCII input is not checked for, and will produce garbage.
pub fn parse_digit_line(line: &str) -> Result<Vec<u8>> {
    if !line.is_ascii() {
        bail!("non-ascii input")
    }
    Ok(line.as_bytes().iter().map(|n| n - 48).collect())
}
