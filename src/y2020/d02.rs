use crate::utils::{Error, Input};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref INPUT_RE: regex::Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

pub fn run(mut input: Input) -> Result<(usize, usize), Error> {
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match INPUT_RE.captures(s) {
            None => Err(Error::NoMatch()),
            Some(captures) => Ok(Entry {
                pos1: usize::from_str(captures.get(1).unwrap().as_str())?,
                pos2: usize::from_str(captures.get(2).unwrap().as_str())?,
                char: char::from_str(captures.get(3).unwrap().as_str())?,
                pass: captures.get(4).unwrap().as_str().to_owned(),
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
