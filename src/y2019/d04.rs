use crate::utils::{CaptureParser, Input};
use anyhow::bail;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INPUT_RE: regex::Regex = Regex::new(r"^(\d{6})-(\d{6})$").unwrap();
}

pub fn run(mut input: Input) -> Result<(u32, u32)> {
    let mut output = (0, 0);

    let input = match input.next() {
        Some(Ok(line)) => line,
        _ => bail!("Empty input"),
    };
    let range = match INPUT_RE.captures(&input) {
        Some(captures) => captures.parse(1)?..=captures.parse(2)?,
        _ => bail!("Invalid input: {}", input),
    };

    // Brute-force all possible values in range, we could be smarter though
    for number in range {
        let valid = validate(number);
        if valid.0 {
            output.0 += 1;
        }
        if valid.1 {
            output.1 += 1;
        }
    }

    Ok(output)
}

fn validate(number: u32) -> (bool, bool) {
    // First part: two adjacent digits are the same
    let mut found_repeat = false;
    // First part: going from left to right, the digits never decrease
    let mut found_decrease = false;
    // Second part: the two adjacent matching digits are not part of a larger group
    let mut found_double = false;

    let mut previous = None;
    let mut repeats = 0;

    for place in 0..6 {
        let current = (number / (10_u32).pow(place)) % 10;
        if let Some(previous) = previous {
            if current == previous {
                found_repeat = true;
                repeats += 1;
            } else {
                if repeats == 1 {
                    found_double = true;
                }
                repeats = 0;
            }
            if current > previous {
                found_decrease = true;
            }
        }
        previous = Some(current);
    }
    if repeats == 1 {
        found_double = true;
    }

    let valid_first = found_repeat && !found_decrease;
    let valid_second = valid_first && found_double;

    (valid_first, valid_second)
}

#[test]
fn test_validate() {
    assert_eq!((false, false), validate(223450)); // Going down
    assert_eq!((false, false), validate(123789)); // No doubles

    assert_eq!((true, false), validate(123444)); // Triple at the end
    assert_eq!((true, false), validate(111123)); // Quadruple at the start

    assert_eq!((true, true), validate(111122));
    assert_eq!((true, true), validate(112233));
}
