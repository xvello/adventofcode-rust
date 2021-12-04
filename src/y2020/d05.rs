use crate::utils::Input;
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::collections::BTreeSet;
use std::str::FromStr;

lazy_static! {
    /// Regexp matching the seat description on the boarding pass
    static ref SEAT_RE: regex::Regex = Regex::new(r"^([BF]{7})([RL]{3})$").unwrap();
}

pub fn run(input: &Input) -> Result<(u16, u16)> {
    let mut output = (0, 0);
    let mut known_ids = BTreeSet::new();
    for line in input.lines() {
        let id = Seat::from_str(line)?.get_id();
        // First result is the highest seat ID
        output.0 = output.0.max(id);
        // Store ID to deduce ours by elimination
        known_ids.insert(id);
    }

    // Our seat is between two known IDs, let's iterate over the sorted set to find a gap
    let mut prev_id = None;
    for id in known_ids {
        if let Some(prev_id) = prev_id {
            if id - prev_id == 2 {
                // We found a gap in the set, let's sit here
                output.1 = id - 1;
                break;
            }
        }
        prev_id.replace(id);
    }

    Ok(output)
}

struct Seat {
    row: u8,
    column: u8,
}

impl FromStr for Seat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match SEAT_RE.captures(s) {
            None => bail!("Invalid format: {}", s),
            Some(captures) => Ok(Self {
                row: Seat::parse_binary(captures.get(1), 'B')?,
                column: Seat::parse_binary(captures.get(2), 'R')?,
            }),
        }
    }
}

impl Seat {
    fn parse_binary(input: Option<Match>, ones: char) -> Result<u8> {
        match input {
            None => bail!("Empty input"),
            Some(input) => {
                let mut result = 0;
                for bit in input.as_str().chars() {
                    result *= 2;
                    if bit == ones {
                        result += 1;
                    }
                }
                Ok(result)
            }
        }
    }

    // Max ID is 1023, u16 is safe
    fn get_id(&self) -> u16 {
        ((self.row as u16) * 8) + (self.column as u16)
    }
}

#[test]
fn test_seat_parsing() -> Result<()> {
    let test_cases = vec![
        ("BFFFBBFRRR", 70, 7, 567),
        ("FFFBBBFRRR", 14, 7, 119),
        ("BBFFBBFRLL", 102, 4, 820),
        ("BBBBBBBRRR", 127, 7, 1023),
    ];

    for (input, row, column, id) in test_cases {
        let seat = Seat::from_str(input)?;
        assert_eq!(row, seat.row);
        assert_eq!(column, seat.column);
        assert_eq!(id, seat.get_id());
    }
    Ok(())
}
