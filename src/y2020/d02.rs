use crate::utils::{CaptureParser, Input};
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref INPUT_RE: regex::Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

pub fn run(mut input: Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);

    while let Some(Ok(line)) = input.next() {
        let entry = Entry::from_str(&line)?;
        if entry.validate_first() {
            output.0 += 1;
        }
        if entry.validate_second() {
            output.1 += 1;
        }
    }
    Ok(output)
}

/// Holds a parsed password entry and its validation params
struct Entry {
    pos1: usize,
    pos2: usize,
    char: char,
    pass: String,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match INPUT_RE.captures(s) {
            None => bail!("Invalid input: {}", s),
            Some(captures) => Ok(Entry {
                pos1: captures.parse(1)?,
                pos2: captures.parse(2)?,
                char: captures.parse(3)?,
                pass: captures.parse(4)?,
            }),
        }
    }
}

impl Entry {
    /// First meaning: char must appear between min and max times
    fn validate_first(&self) -> bool {
        let count = self.pass.matches(self.char).count();
        count >= self.pos1 && count <= self.pos2
    }

    /// Second meaning: the char must be at exactly one of the positions
    fn validate_second(&self) -> bool {
        let matches = self
            .pass
            .match_indices(self.char)
            .filter(|(p, _)| *p + 1 == self.pos1 || *p + 1 == self.pos2)
            .count();
        matches == 1
    }
}
