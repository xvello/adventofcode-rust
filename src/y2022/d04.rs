use crate::utils::{CaptureParser, Input};
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref INPUT_RE: regex::Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
}
pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);
    let assignments: Vec<AssignmentPair> = input.lines_into()?;
    output.0 = assignments.iter().filter(|a| a.has_full_overlap()).count();
    output.1 = assignments.iter().filter(|a| a.has_some_overlap()).count();

    Ok(output)
}

struct AssignmentPair {
    left: (usize, usize),
    right: (usize, usize),
}

impl FromStr for AssignmentPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match INPUT_RE.captures(s) {
            Some(captures) => Ok(AssignmentPair {
                left: (captures.parse(1)?, captures.parse(2)?),
                right: (captures.parse(3)?, captures.parse(4)?),
            }),
            _ => bail!("invalid input shape"),
        }
    }
}

impl AssignmentPair {
    fn has_full_overlap(&self) -> bool {
        self.left.0 >= self.right.0 && self.left.1 <= self.right.1
            || self.left.0 <= self.right.0 && self.left.1 >= self.right.1
    }
    fn has_some_overlap(&self) -> bool {
        self.left.0 <= self.right.1 && self.left.1 >= self.right.0
    }
}
