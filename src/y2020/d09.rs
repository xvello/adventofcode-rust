use crate::utils::Input;
use anyhow::{bail, Result};
use std::str::FromStr;

pub fn run(mut input: Input) -> Result<(usize, usize)> {
    let mut past_values = vec![];
    let mut invalid_value = 0;

    while let Some(Ok(line)) = input.next() {
        let value = usize::from_str(&line)?;
        if !is_valid(&past_values, value) {
            log::debug!(
                "Found invalid value {} position {}",
                value,
                past_values.len()
            );
            invalid_value = value;
            break;
        }
        past_values.push(value)
    }
    Ok((invalid_value, find_weak(&past_values, invalid_value)?))
}

/// Check that a given value is the sum of two of the last 25 values
fn is_valid(buffer: &[usize], value: usize) -> bool {
    if buffer.len() < 25 {
        return true; // Preamble
    }
    let last_25 = &buffer[(buffer.len() - 25)..buffer.len()];
    for x in last_25.iter() {
        for y in last_25.iter() {
            if x == y {
                continue;
            }
            if x + y == value {
                return true;
            }
        }
    }
    false
}

/// Brute-force past values to find the encryption weakness
fn find_weak(buffer: &[usize], value: usize) -> Result<usize> {
    for start in 0..buffer.len() {
        for end in start + 2..buffer.len() {
            let sum: usize = buffer[start..=end].iter().sum();
            if sum == value {
                // We found our weakness
                log::debug!("Found that sum [{}..={}] == {}", start, end, value);
                let min = buffer[start..=end].iter().min().unwrap();
                let max = buffer[start..=end].iter().max().unwrap();
                return Ok(min + max);
            }
        }
    }
    bail!("No match found")
}
