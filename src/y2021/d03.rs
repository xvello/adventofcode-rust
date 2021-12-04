use crate::utils::Input;
use anyhow::{bail, Ok, Result};
use std::ops::{AddAssign, SubAssign};

const REPORT_SIZE: usize = 12;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let values: Vec<u16> = input
        .lines()
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect();

    Ok((compute_first(&values)?, compute_second(&values)?))
}

fn compute_first(values: &[u16]) -> Result<usize> {
    let mut seen: [i16; REPORT_SIZE] = [0; REPORT_SIZE];
    for v in values {
        for (i, s) in seen.iter_mut().enumerate() {
            if is_set(v, &i) {
                s.add_assign(1)
            } else {
                s.sub_assign(1)
            }
        }
    }
    let (mut gamma, mut epsilon) = (0_usize, 0_usize);
    for i in 0..REPORT_SIZE {
        gamma <<= 1;
        epsilon <<= 1;
        if seen[REPORT_SIZE - 1 - i] > 0 {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }
    Ok(gamma * epsilon)
}

fn compute_second(values: &[u16]) -> Result<usize> {
    let o2 = find_value(values, REPORT_SIZE - 1, true)?;
    let co2 = find_value(values, REPORT_SIZE - 1, false)?;
    Ok(o2 * co2)
}

// Recursively filter out the values until one is left. Position is 0-indexed.
fn find_value(values: &[u16], position: usize, criteria: bool) -> Result<usize> {
    let mut seen = 0_i16;
    for v in values {
        if is_set(v, &position) {
            seen += 1;
        } else {
            seen -= 1;
        }
    }
    let filtered: Vec<u16> = values
        .iter()
        .filter(|v| {
            if seen == 0 {
                is_set(v, &position) == criteria
            } else {
                is_set(v, &position) == (criteria == (seen > 0))
            }
        })
        .copied()
        .collect();
    if filtered.len() == 1 {
        Ok(filtered[0].into())
    } else if position == 0 {
        bail!("cannot find unique value, remaining: {:?}", filtered)
    } else {
        find_value(&filtered, position - 1, criteria)
    }
}

#[inline]
fn is_set(v: &u16, position: &usize) -> bool {
    (v & 1 << position) != 0
}

#[test]
fn test_find_value() -> Result<()> {
    let values = [
        0b00100u16, 0b11110u16, 0b10110u16, 0b10111u16, 0b10101u16, 0b01111u16, 0b00111u16,
        0b11100u16, 0b10000u16, 0b11001u16, 0b00010u16, 0b01010u16,
    ];
    assert_eq!(23, find_value(&values, 4, true).unwrap());
    assert_eq!(10, find_value(&values, 4, false).unwrap());
    Ok(())
}
